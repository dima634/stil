use super::Hyprland;
use serde::{Deserialize, de::DeserializeOwned};
use std::{fmt::Display, io::Read, os::unix::net::UnixStream};
use tracing::warn;

#[derive(Debug)]
pub enum EventParseErr {
    UnknownEvent(String),
    InvalidData,
    Malformed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Address(pub usize);

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}", self.0)
    }
}

impl<'de> Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        let address = usize::from_str_radix(s, 16).map_err(serde::de::Error::custom)?;
        Ok(Address(address))
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ActiveWindowV2 {
    pub address: Address,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CloseWindow {
    pub window_address: Address,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MoveWindowV2 {
    pub window_address: Address,
    pub workspace_id: i32,
    pub workspace_name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WorkspaceV2 {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OpenWindow {
    pub window_address: Address,
    pub workspace_name: String,
    pub window_class: String,
    pub window_title: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ActiveLayout {
    pub keyboard_name: String,
    pub layout_name: String,
}

#[derive(Debug, Clone)]
pub enum Event {
    OpenWindow(OpenWindow),
    CloseWindow(CloseWindow),
    ActiveWindowV2(ActiveWindowV2),
    CreateWorkspace(WorkspaceV2),
    DestroyWorkspace(WorkspaceV2),
    FocusWorkspace(WorkspaceV2),
    MoveWindowV2(MoveWindowV2),
    ActiveLayout(ActiveLayout),
}

impl TryFrom<&String> for Event {
    type Error = EventParseErr;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let (name, data) = value.split_once(">>").ok_or(EventParseErr::Malformed)?;

        let event = match name {
            "openwindow" => Event::OpenWindow(parse_event(data)?),
            "closewindow" => Event::CloseWindow(parse_event(data)?),
            "activewindowv2" => Event::ActiveWindowV2(parse_event(data)?),
            "movewindowv2" => Event::MoveWindowV2(parse_event(data)?),
            "workspacev2" => Event::FocusWorkspace(parse_event(data)?),
            "createworkspacev2" => Event::CreateWorkspace(parse_event(data)?),
            "destroyworkspacev2" => Event::DestroyWorkspace(parse_event(data)?),
            "activelayout" => Event::ActiveLayout(parse_event(data)?),
            _ => return Err(EventParseErr::UnknownEvent(name.to_string())),
        };

        Ok(event)
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

fn parse_event<T: DeserializeOwned>(data: &str) -> Result<T, EventParseErr> {
    csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(data.as_bytes())
        .deserialize::<T>()
        .next()
        .ok_or(EventParseErr::InvalidData)?
        .map_err(|_| EventParseErr::InvalidData)
}
