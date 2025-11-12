use crate::{
    services::{global_init, system_event_dispatcher, workspace_registry},
    system_events::SystemEvent,
    workspaces::{Workspace, WorkspaceRegistry},
};
use std::sync::MutexGuard;

#[cxx::bridge]
mod workspaces {
    extern "Rust" {
        type Workspace;

        fn id(&self) -> i32;
        fn name(&self) -> &String;
    }

    extern "Rust" {
        type Workspaces;

        #[Self = "Workspaces"]
        fn lock() -> Box<Workspaces>;
        fn all(&self) -> &[Workspace];
        fn current_workspace_id(&self) -> i32;
    }

    pub enum EventKind {
        WorkspaceCreated,
        WorkspaceDestroyed,
        WorkspaceFocused,
        Unknown,
    }

    extern "Rust" {
        type Event;

        fn kind(&self) -> EventKind;
    }

    extern "Rust" {
        type SystemEvents;

        #[Self = "SystemEvents"]
        fn create() -> Box<SystemEvents>;
        fn next(&self) -> Box<Event>;
    }
}

struct Workspaces(MutexGuard<'static, WorkspaceRegistry>);

impl Workspaces {
    pub fn lock() -> Box<Self> {
        Box::new(Self(workspace_registry()))
    }

    pub fn all(&self) -> &[Workspace] {
        self.0.workspaces()
    }

    pub fn current_workspace_id(&self) -> i32 {
        self.0.current_workspace_id()
    }
}

struct Event(SystemEvent);

impl Event {
    pub fn kind(&self) -> workspaces::EventKind {
        use crate::hyprland::Event as HyprEvent;
        use workspaces::EventKind;

        match &self.0 {
            SystemEvent::Hyprland(HyprEvent::DestroyWorkspace(_)) => EventKind::WorkspaceDestroyed,
            SystemEvent::Hyprland(HyprEvent::CreateWorkspace(_)) => EventKind::WorkspaceCreated,
            SystemEvent::Hyprland(HyprEvent::FocusWorkspace(_)) => EventKind::WorkspaceFocused,
            _ => EventKind::Unknown,
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
