mod active_window;
mod close_window;
mod open_window;

pub use active_window::ActiveWindow;
pub use close_window::CloseWindow;
pub use open_window::OpenWindow;

use super::Hyprland;
use std::{
    collections::HashMap,
    io::Read,
    os::unix::net::UnixStream,
    sync::{
        Mutex,
        mpsc::{Receiver, Sender},
    },
};
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
}

impl TryFrom<&String> for Event {
    type Error = EventParseErr;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let (name, data) = value.split_once(">>").ok_or(EventParseErr::Malformed)?;

        match name {
            "openwindow" => Ok(Event::OpenWindow(OpenWindow::try_from(data)?)),
            "closewindow" => Ok(Event::CloseWindow(CloseWindow::try_from(data)?)),
            "activewindow" => Ok(Event::ActiveWindow(ActiveWindow::try_from(data)?)),
            _ => Err(EventParseErr::UnknownEvent(name.to_string())),
        }
    }
}

#[derive(Debug, Default)]
pub struct HyprEvents {
    next_listener_id: Mutex<u32>,
    subs: Mutex<HashMap<u32, Sender<Event>>>,
}

impl HyprEvents {
    pub fn add_listener(&self) -> Option<Receiver<Event>> {
        let (tx, rx) = std::sync::mpsc::channel();
        let mut subs = self.subs.lock().ok()?;
        let mut id = self.next_listener_id.lock().ok()?;
        subs.insert(*id, tx);
        *id += 1;
        Some(rx)
    }

    /// Start listening for Hyprland events. This operation is blocking.
    pub fn listen(&self) -> Option<()> {
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
                        let Ok(mut subs) = self.subs.lock() else { return None };
                        let mut closed_channels = Vec::new();

                        for (&id, tx) in subs.iter() {
                            if let Err(_) = tx.send(event.clone()) {
                                closed_channels.push(id);
                            }
                        }

                        for id in closed_channels {
                            subs.remove(&id);
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
