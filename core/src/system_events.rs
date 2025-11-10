use std::sync::{Arc, LazyLock, Mutex, mpsc};

#[derive(Debug, Copy, Clone)]
pub enum SystemEvent {}

pub struct SystemEventDispatcher {
    in_tx: mpsc::Sender<SystemEvent>,
    out_txs: Mutex<Vec<mpsc::Sender<SystemEvent>>>,
}

impl SystemEventDispatcher {

    #[inline]
    pub fn tx(&self) -> mpsc::Sender<SystemEvent> {
        self.in_tx.clone()
    }

    pub fn rx(&self) -> mpsc::Receiver<SystemEvent> {
        let (tx, rx) = mpsc::channel();
        let mut out_txs = self.out_txs.lock().expect("should not be poisoned");
        out_txs.push(tx);
        rx
    }

    fn new() -> (Self, mpsc::Receiver<SystemEvent>) {
        let (in_tx, in_rx) = mpsc::channel();
        (
            Self {
                in_tx,
                out_txs: Mutex::new(Vec::new()),
            },
            in_rx,
        )
    }

    fn listen(&self, rx: mpsc::Receiver<SystemEvent>) {
        for event in rx.iter() {
            // Handle different system events here

            let mut out_txs = self.out_txs.lock().expect("should not be poisoned");
            out_txs.retain(|tx| tx.send(event.clone()).is_ok());
        }
    }
}

pub const SYSTEM_EVENT_DISPATCHER: LazyLock<Arc<SystemEventDispatcher>> =
    LazyLock::new(|| {
        let (dispatcher, rx) = SystemEventDispatcher::new();
        let dispatcher = Arc::new(dispatcher);
        let dispatcher_listener = Arc::clone(&dispatcher);

        std::thread::spawn(move || {
            dispatcher_listener.listen(rx);
        });

        dispatcher
    });
