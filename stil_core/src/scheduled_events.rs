use crate::system_events::{CpuUsage, SystemEvent};
use std::sync::{LazyLock, Mutex, mpsc::Sender};

pub trait ScheduledTask {
    fn update(&mut self, tx: &Sender<SystemEvent>) -> bool;
}

pub struct TaskScheduler {
    tx: Sender<SystemEvent>,
    interval: std::time::Duration,
    events: Vec<Box<dyn ScheduledTask>>,
}

impl TaskScheduler {
    #[inline]
    pub fn new(tx: Sender<SystemEvent>) -> Self {
        Self {
            tx,
            interval: std::time::Duration::from_secs(1),
            events: Vec::new(),
        }
    }

    #[inline]
    pub fn with_interval(mut self, interval: std::time::Duration) -> Self {
        self.interval = interval;
        self
    }

    #[inline]
    pub fn add_task<T: ScheduledTask + 'static>(mut self, task: T) -> Self {
        self.events.push(Box::new(task));
        self
    }

    pub fn run(mut self) {
        loop {
            for event in self.events.iter_mut() {
                if !event.update(&self.tx) {
                    return;
                }
            }

            std::thread::sleep(self.interval);
        }
    }
}

#[derive(Debug)]
pub struct SystemTimeTask;

impl ScheduledTask for SystemTimeTask {
    fn update(&mut self, tx: &Sender<SystemEvent>) -> bool {
        let now = std::time::SystemTime::now();
        let secs_since_unix = now.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
        tx.send(SystemEvent::SystemTimeUpdated(secs_since_unix)).is_ok()
    }
}

#[derive(Debug)]
pub struct CpuTask;

impl ScheduledTask for CpuTask {
    fn update(&mut self, tx: &Sender<SystemEvent>) -> bool {
        let mut system = system();
        system.refresh_cpu_usage();

        let mut usage = CpuUsage {
            cores: [0.0; _],
            num_cores: system.cpus().len(),
            total: system.global_cpu_usage(),
        };

        for (i, cpu) in system.cpus().iter().enumerate() {
            usage.cores[i] = cpu.cpu_usage();
        }

        tx.send(SystemEvent::CpuUsageUpdated(usage)).is_ok()
    }
}

static SYSTEM: LazyLock<Mutex<sysinfo::System>> = LazyLock::new(|| Mutex::new(sysinfo::System::new()));

#[inline]
fn system() -> std::sync::MutexGuard<'static, sysinfo::System> {
    SYSTEM.lock().expect("should not be poisoned")
}
