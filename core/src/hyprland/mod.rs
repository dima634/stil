mod ctl;
mod events;

pub use ctl::*;
pub use events::*;

use std::path::PathBuf;

#[derive(Debug)]
pub struct Hyprland;

impl Hyprland {
    pub fn socket_directory() -> Option<PathBuf> {
        let mut xdg_runtime_dir: PathBuf = std::env::var("XDG_RUNTIME_DIR").ok()?.into();
        let his = std::env::var("HYPRLAND_INSTANCE_SIGNATURE").ok()?;

        xdg_runtime_dir.push("hypr");
        xdg_runtime_dir.push(his);
        Some(xdg_runtime_dir)
    }

    #[inline]
    pub fn events_socket_path() -> Option<PathBuf> {
        Self::socket_directory().map(|dir| dir.join(".socket2.sock"))
    }

    #[inline]
    pub fn ctl_socket_path() -> Option<PathBuf> {
        Self::socket_directory().map(|dir| dir.join(".socket.sock"))
    }
}
