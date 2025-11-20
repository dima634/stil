use crate::{
    services::{global_init, system_event_dispatcher},
    system_events::{SystemEvent, WindowMoved, WindowOpened, WorkspaceCreated},
};

#[cxx::bridge(namespace = "core")]
mod ffi {
    pub enum EventKind {
        WorkspaceCreated,
        WorkspaceDestroyed,
        WorkspaceFocused,
        WindowOpen,
        WindowClose,
        WindowMoved,
        Empty,
    }

    #[namespace = "core"]
    extern "C++" {
        include!("stil_core/src/system_events.rs.h");
        type WindowOpened = crate::system_events::WindowOpened;
        type WorkspaceCreated = crate::system_events::WorkspaceCreated;
        type WindowMoved = crate::system_events::WindowMoved;
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
            _ => EventKind::Empty,
        }
    }
}

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
}

struct SystemEvents {
    rx: std::sync::mpsc::Receiver<SystemEvent>,
}

impl SystemEvents {
    pub fn create() -> Box<Self> {
        global_init();
        let rx = system_event_dispatcher().rx();
        Box::new(Self { rx })
    }

    pub fn next(&self) -> Box<Event> {
        self.rx.recv().map(|event| Box::new(Event(event))).unwrap()
    }
}
