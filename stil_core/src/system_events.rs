use std::sync::{Mutex, mpsc};

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

#[derive(Debug, Clone)]
pub enum SystemEvent {
    // Hyprland related events
    WorkspaceCreated(WorkspaceCreated),
    WorkspaceDestroyed(i32),
    WorkspaceFocused(i32),
    WindowOpened(WindowOpened),
    WindowClosed(usize), // TODO: change to Address
    WindowFocused(usize),
    WindowMoved(WindowMoved),
    KeyboardLayoutChanged(String),
    Empty,
}
