mod application;
mod db;
mod ffi;
mod freedesktop;
mod hyprland;
mod init;
mod repos;
mod system;
mod system_events;
mod workspaces;

pub use db::{migrate_down, migrate_up};
pub use init::init;

#[cxx::bridge(namespace = "core")]
mod libffi {
    extern "Rust" {
        fn init();
    }
}
