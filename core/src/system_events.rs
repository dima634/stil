use crate::{hyprland, services::workspace_registry};
use std::sync::{Mutex, mpsc};

pub trait SystemEventConsumer {
    fn consume(&mut self, event: &SystemEvent);
}

#[derive(Debug, Clone)]
pub enum SystemEvent {
    Hyprland(hyprland::Event),
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
            workspace_registry().consume(&event);

            let mut out_txs = self.out_txs.lock().expect("should not be poisoned");
            out_txs.retain(|tx| tx.send(event.clone()).is_ok());
        }
    }
}
