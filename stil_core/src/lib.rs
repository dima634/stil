mod application;
mod ffi;
mod freedesktop;
mod hyprland;
mod services;
mod system;
mod system_events;
mod workspaces;

pub use services::init;

#[cxx::bridge(namespace = "core")]
mod libffi {
    extern "Rust" {
        fn init();
    }
}
