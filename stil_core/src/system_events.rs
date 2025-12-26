use crate::Address;

#[derive(Debug, Clone)]
pub struct WindowOpened {
    pub address: Address,
    pub workspace_id: i32,
    pub app_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct WindowClosed {
    pub address: Address,
    pub workspace_id: i32,
}

#[derive(Debug, Clone)]
pub struct WorkspaceCreated {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct WindowMoved {
    pub window_address: Address,
    pub from_workspace: i32,
    pub to_workspace: i32,
}

#[derive(Debug, Clone)]
pub struct Notification {
    pub summary: String,
    pub body: Option<String>,
    pub icon: Option<String>,
    pub image: Option<Image>,
}

#[derive(Debug, Clone)]
pub struct Image {
    pub data: Vec<u8>,
    pub has_alpha: bool,
    pub bits_per_sample: i32,
    pub width: i32,
    pub height: i32,
    pub row_stride: i32,
    pub channels: i32,
}

#[derive(Debug, Clone)]
pub enum SystemEvent {
    // Hyprland related events
    WorkspaceCreated(WorkspaceCreated),
    WorkspaceDestroyed(i32),
    WorkspaceOpened(i32),
    WindowOpened(WindowOpened),
    WindowClosed(WindowClosed),
    WindowFocused(Address),
    WindowMoved(WindowMoved),
    KeyboardLayoutChanged(&'static str),
    Notification(Notification),
    Empty,
}
