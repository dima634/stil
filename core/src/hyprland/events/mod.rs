mod active_window;
mod close_window;
mod open_window;
mod workspace_v2;

pub use active_window::ActiveWindow;
pub use close_window::CloseWindow;
pub use open_window::OpenWindow;
pub use workspace_v2::*;

use super::Hyprland;
use std::{io::Read, os::unix::net::UnixStream};
use tracing::warn;

#[derive(Debug)]
pub enum EventParseErr {
    UnknownEvent(String),
    InvalidData,
    Malformed,
}

#[derive(Debug, Clone)]
pub enum Event {
    OpenWindow(OpenWindow),
    CloseWindow(CloseWindow),
    ActiveWindow(ActiveWindow),
    CreateWorkspace(CreateWorkspaceV2),
    DestroyWorkspace(DestroyWorkspaceV2),
}

impl TryFrom<&String> for Event {
    type Error = EventParseErr;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let (name, data) = value.split_once(">>").ok_or(EventParseErr::Malformed)?;

        match name {
            "openwindow" => Ok(Event::OpenWindow(OpenWindow::try_from(data)?)),
            "closewindow" => Ok(Event::CloseWindow(CloseWindow::try_from(data)?)),
            "activewindow" => Ok(Event::ActiveWindow(ActiveWindow::try_from(data)?)),
            "createworkspacev2" => Ok(Event::CreateWorkspace(CreateWorkspaceV2::try_from(data)?)),
            "destroyworkspacev2" => Ok(Event::DestroyWorkspace(DestroyWorkspaceV2::try_from(data)?)),
            _ => Err(EventParseErr::UnknownEvent(name.to_string())),
        }
    }
}

#[derive(Debug, Default)]
pub struct HyprEvents;

impl HyprEvents {
    /// Start listening for Hyprland events. This operation is blocking.
    pub fn listen(mut callback: impl FnMut(Event) -> bool) -> Option<()> {
        let mut buffer = [0u8; 1024];
        let mut raw_event = String::new();

        let Some(socket_path) = Hyprland::events_socket_path() else { return None };
        let Ok(mut socket) = UnixStream::connect(socket_path) else { return None };

        loop {
            let num_bytes = match socket.read(&mut buffer) {
                Ok(len) => len,
                Err(err) => {
                    warn!("Failed to read from Hyprland events socket: {}", err.kind());
                    raw_event.clear();
                    continue;
                }
            };

            let Ok(event_part) = std::str::from_utf8(&buffer[..num_bytes]) else {
                warn!("Received invalid UTF-8 data from Hyprland events socket");
                raw_event.clear();
                continue;
            };

            for char in event_part.chars() {
                if char == '\n' {
                    if let Ok(event) = Event::try_from(&raw_event) {
                        if !callback(event) {
                            return Some(());
                        }
                    }

                    raw_event.clear();
                } else {
                    raw_event.push(char);
                }
            }
        }
    }
}
