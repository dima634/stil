mod active_window;
mod close_window;
mod open_window;

pub use active_window::ActiveWindow;
pub use close_window::CloseWindow;
pub use open_window::OpenWindow;

use super::Hyprland;
use std::{cell::RefCell, io::Read, os::unix::net::UnixStream, rc::Rc, sync::mpsc::Sender, time::Duration};
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

#[derive(Debug)]
pub struct HyprEvents {
    subs: Vec<Sender<Event>>,
}

impl HyprEvents {
    pub fn listen(cancelled: Rc<RefCell<bool>>, mut callback: impl FnMut(Event)) {
        let mut buffer = [0u8; 1024];
        let mut raw_event = String::new();

        let Some(socket_path) = Hyprland::events_socket_path() else { return };
        let Ok(mut socket) = UnixStream::connect(socket_path) else { return };
        socket.set_read_timeout(Some(Duration::from_secs(2))).expect("duration is not zero");

        loop {
            let num_bytes = match socket.read(&mut buffer) {
                Ok(len) => len,
                Err(err) if err.kind() == std::io::ErrorKind::WouldBlock => {
                    if *cancelled.borrow() {
                        return;
                    } else {
                        continue;
                    }
                },
                Err(err) => {
                    warn!("Failed to read from Hyprland events socket: {}", err.kind());
                    raw_event.clear();
                    continue;
                }
            };

            if *cancelled.borrow() {
                return;
            }

            let Ok(event_part) = std::str::from_utf8(&buffer[..num_bytes]) else {
                warn!("Received invalid UTF-8 data from Hyprland events socket");
                raw_event.clear();
                continue;
            };

            for char in event_part.chars() {
                if char == '\n' {
                    if let Ok(event) = Event::try_from(&raw_event) {
                        callback(event);
                    }

                    raw_event.clear();
                } else {
                    raw_event.push(char);
                }
            }
        }
    }
}
