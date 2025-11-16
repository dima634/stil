use sysinfo::CpuRefreshKind;
use tracing::info;

use crate::system_events::SystemEvent;
use std::sync::{LazyLock, Mutex, mpsc::Sender};

pub trait ScheduledEvent {
    fn update(&mut self, tx: &Sender<SystemEvent>) -> bool;
}

pub struct Scheduler {
    tx: Sender<SystemEvent>,
    interval: std::time::Duration,
    events: Vec<Box<dyn ScheduledEvent>>,
}

impl Scheduler {
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
    pub fn add_event<T: ScheduledEvent + 'static>(mut self, event: T) -> Self {
        self.events.push(Box::new(event));
        self
    }

    pub fn run(self) {
        for mut event in self.events {
            event.update(&self.tx);
        }

        std::thread::sleep(self.interval);
    }
}

#[derive(Debug)]
pub struct SystemTimeEvent;

impl ScheduledEvent for SystemTimeEvent {
    fn update(&mut self, tx: &Sender<SystemEvent>) -> bool {
        let now = std::time::SystemTime::now();
        tx.send(SystemEvent::SystemTimeUpdated(now)).is_ok()
    }
}

#[derive(Debug)]
pub struct CpuEvent;

impl ScheduledEvent for CpuEvent {
    fn update(&mut self, tx: &Sender<SystemEvent>) -> bool {
        let mut system = system();
        system.refresh_cpu_usage();
        info!("CPU usage: {:?}", system.global_cpu_usage());
        tx.send(SystemEvent::Empty).is_ok()
    }
}

static SYSTEM: LazyLock<Mutex<sysinfo::System>> = LazyLock::new(|| Mutex::new(sysinfo::System::new()));

#[inline]
fn system() -> std::sync::MutexGuard<'static, sysinfo::System> {
    SYSTEM.lock().expect("should not be poisoned")
}
