use std::sync::{Mutex, mpsc};

mod window_events;
mod workspace_events;

pub use window_events::*;
pub use workspace_events::*;

#[derive(Debug, Clone)]
pub enum SystemEvent {
    WorkspaceCreated(WorkspaceCreated),
    WorkspaceDestroyed(i32),
    WorkspaceFocused(i32),
    WindowOpened(WindowOpened),
    WindowClosed(usize),
    WindowFocused(usize),
    Invalid,
}

pub struct SystemEventDispatcher {
    in_tx: mpsc::Sender<SystemEvent>,
    out_txs: Mutex<Vec<mpsc::Sender<SystemEvent>>>,
}

impl SystemEventDispatcher {
    pub fn new() -> (Self, mpsc::Receiver<SystemEvent>) {
        let (in_tx, in_rx) = mpsc::channel();
        (
            Self {
                in_tx,
                out_txs: Mutex::new(Vec::new()),
            },
            in_rx,
        )
    }

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

    pub fn listen(&self, rx: mpsc::Receiver<SystemEvent>) {
        for event in rx.iter() {
            // Delegate handling of event to corresponding services
            // TODO ...

            let mut out_txs = self.out_txs.lock().expect("should not be poisoned");
            out_txs.retain(|tx| tx.send(event.clone()).is_ok());
        }
    }
}
