use crate::services::system_event_dispatcher;
use std::sync::{Mutex, mpsc};

pub use ffi::{WindowMoved, WindowOpened, WorkspaceCreated};

#[derive(Debug, Clone)]
pub enum SystemEvent {
    // Hyprland related events
    WorkspaceCreated(WorkspaceCreated),
    WorkspaceDestroyed(i32),
    WorkspaceFocused(i32),
    WindowOpened(WindowOpened),
    WindowClosed(usize),
    WindowFocused(usize),
    WindowMoved(WindowMoved),
    Empty,
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

#[cxx::bridge(namespace = "core")]
mod ffi {
    #[derive(Debug, Clone)]
    pub struct WindowOpened {
        pub address: usize,
        pub workspace_name: String,
        pub class_name: String,
    }

    #[derive(Debug, Clone)]
    pub struct WorkspaceCreated {
        pub id: i32,
        pub name: String,
    }

    #[derive(Debug, Clone)]
    pub struct WindowMoved {
        pub address: usize,
        pub workspace_id: i32,
        pub workspace_name: String,
    }

    pub enum EventKind {
        WorkspaceCreated,
        WorkspaceDestroyed,
        WorkspaceFocused,
        WindowOpen,
        WindowClose,
        WindowMoved,
        WindowFocused,
        Empty,
    }

    extern "Rust" {
        type Event;
        fn kind(&self) -> EventKind;

        fn workspace_created(&mut self) -> Result<WorkspaceCreated>;
        fn workspace_destroyed(&mut self) -> Result<i32>;
        fn workspace_focused(&mut self) -> Result<i32>;
        fn window_opened(&mut self) -> Result<WindowOpened>;
        fn window_closed(&mut self) -> Result<usize>;
        fn window_moved(&mut self) -> Result<WindowMoved>;
        fn window_focused(&mut self) -> Result<usize>;
    }

    extern "Rust" {
        type SystemEvents;

        #[Self = "SystemEvents"]
        fn create() -> Box<SystemEvents>;
        fn next(&self) -> Box<Event>;
    }
}

impl From<&SystemEvent> for ffi::EventKind {
    fn from(event: &SystemEvent) -> Self {
        use ffi::EventKind;

        match event {
            SystemEvent::WorkspaceDestroyed(_) => EventKind::WorkspaceDestroyed,
            SystemEvent::WorkspaceCreated(_) => EventKind::WorkspaceCreated,
            SystemEvent::WorkspaceFocused(_) => EventKind::WorkspaceFocused,
            SystemEvent::WindowOpened(_) => EventKind::WindowOpen,
            SystemEvent::WindowClosed(_) => EventKind::WindowClose,
            SystemEvent::WindowMoved(_) => EventKind::WindowMoved,
            SystemEvent::WindowFocused(_) => EventKind::WindowFocused,
            _ => EventKind::Empty,
        }
    }
}

/// Wrapper around SystemEvent for FFI
struct Event(SystemEvent);

macro_rules! event_accessor {
    ($name: ident, $result: ident, $ev: ident) => {
        pub fn $name(&mut self) -> Result<$result, &'static str> {
            let mut empty = SystemEvent::Empty;
            std::mem::swap(&mut empty, &mut self.0);

            if let SystemEvent::$ev(payload) = empty {
                Ok(payload)
            } else {
                Err("wrong event kind")
            }
        }
    };
}

impl Event {
    #[inline]
    pub fn kind(&self) -> ffi::EventKind {
        (&self.0).into()
    }

    event_accessor!(workspace_destroyed, i32, WorkspaceDestroyed);
    event_accessor!(workspace_created, WorkspaceCreated, WorkspaceCreated);
    event_accessor!(workspace_focused, i32, WorkspaceFocused);
    event_accessor!(window_opened, WindowOpened, WindowOpened);
    event_accessor!(window_closed, usize, WindowClosed);
    event_accessor!(window_moved, WindowMoved, WindowMoved);
    event_accessor!(window_focused, usize, WindowFocused);
}

/// Wrapper around SystemEventDispatcher for FFI
struct SystemEvents {
    rx: std::sync::mpsc::Receiver<SystemEvent>,
}

impl SystemEvents {
    pub fn create() -> Box<Self> {
        let rx = system_event_dispatcher().rx();
        Box::new(Self { rx })
    }

    pub fn next(&self) -> Box<Event> {
        self.rx.recv().map(|event| Box::new(Event(event))).unwrap()
    }
}
