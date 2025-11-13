use crate::{
    hyprland::{Event as HyprEvent, WorkspaceV2},
    services::{global_init, system_event_dispatcher},
    system_events::SystemEvent,
};

#[cxx::bridge(namespace = "core")]
mod ffi {
    pub enum EventKind {
        WorkspaceCreated,
        WorkspaceDestroyed,
        WorkspaceFocused,
        Unknown,
    }

    extern "Rust" {
        type WorkspaceV2;
        fn id(self: &WorkspaceV2) -> i32;
        fn name(self: &WorkspaceV2) -> &String;
    }

    extern "Rust" {
        type Event;

        fn kind(&self) -> EventKind;
        unsafe fn workspace_created<'ev>(&'ev self) -> Result<&'ev WorkspaceV2>;
        fn workspace_destroyed(&self) -> Result<i32>;
        fn workspace_focused(&self) -> Result<i32>;
    }

    extern "Rust" {
        type SystemEvents;

        #[Self = "SystemEvents"]
        fn create() -> Box<SystemEvents>;
        fn next(&self) -> Box<Event>;
    }
}

impl From<&HyprEvent> for ffi::EventKind {
    fn from(event: &HyprEvent) -> Self {
        use ffi::EventKind;

        match event {
            HyprEvent::DestroyWorkspace(_) => EventKind::WorkspaceDestroyed,
            HyprEvent::CreateWorkspace(_) => EventKind::WorkspaceCreated,
            HyprEvent::FocusWorkspace(_) => EventKind::WorkspaceFocused,
            _ => EventKind::Unknown,
        }
    }
}

struct Event(SystemEvent);

impl Event {
    pub fn kind(&self) -> ffi::EventKind {
        use ffi::EventKind;

        match &self.0 {
            SystemEvent::Hyprland(event) => event.into(),
            _ => EventKind::Unknown,
        }
    }

    pub fn workspace_destroyed(&self) -> Result<i32, &'static str> {
        match &self.0 {
            SystemEvent::Hyprland(HyprEvent::DestroyWorkspace(event)) => Ok(event.id()),
            _ => Err("wrong event kind"),
        }
    }

    pub fn workspace_created(&self) -> Result<&WorkspaceV2, &'static str> {
        match &self.0 {
            SystemEvent::Hyprland(HyprEvent::CreateWorkspace(event)) => Ok(event),
            _ => Err("wrong event kind"),
        }
    }

    pub fn workspace_focused(&self) -> Result<i32, &'static str> {
        match &self.0 {
            SystemEvent::Hyprland(HyprEvent::FocusWorkspace(event)) => Ok(event.id()),
            _ => Err("wrong event kind"),
        }
    }
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
