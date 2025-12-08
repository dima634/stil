use crate::hyprland;
use crate::service_locator::ServiceLocator;

#[cxx::bridge(namespace = "core::desktop")]
mod ffi {
    struct Workspace {
        pub id: i32,
        pub name: String,
    }

    struct Window {
        pub address: usize,
        pub class_name: String,
        pub icon: String,
        pub is_focused: bool,
    }

    struct App {
        pub id: String,
        pub name: String,
        pub icon: String,
    }

    extern "Rust" {
        /// Returns list of windows (addresses) in the given workspace
        fn get_workspace_windows(workspace_id: i32) -> Vec<usize>;
        fn get_workspaces() -> Vec<Workspace>;
        fn get_current_workspace_id() -> i32;
        fn get_window(address: usize) -> Result<Window>;
        fn get_pinned_apps() -> Vec<App>;
        fn launch_app(app_id: &str) -> bool;
    }
}

fn get_current_workspace_id() -> i32 {
    ServiceLocator::desktop().get_current_workspace_id()
}

fn get_workspaces() -> Vec<ffi::Workspace> {
    ServiceLocator::desktop()
        .get_workspaces()
        .map(|ws| ffi::Workspace {
            id: ws.id(),
            name: ws.name().clone(),
        })
        .collect()
}

fn get_workspace_windows(workspace_id: i32) -> Vec<usize> {
    ServiceLocator::desktop()
        .get_workspace_windows(workspace_id)
        .map(|window| window.address().0)
        .collect()
}

fn get_window(address: usize) -> Result<ffi::Window, &'static str> {
    let desktop = ServiceLocator::desktop();
    desktop
        .get_window(hyprland::Address(address))
        .map(|window| {
            let icon = window
                .app_id()
                .map(|app_id| desktop.get_app(app_id))
                .flatten()
                .map(|app| app.icon_path())
                .flatten()
                .map(|icon_path| icon_path.to_string_lossy().to_string())
                .unwrap_or_default();
            ffi::Window {
                icon,
                address,
                class_name: window.class().clone(),
                is_focused: window.is_focused(),
            }
        })
        .ok_or("window not found")
}

fn get_pinned_apps() -> Vec<ffi::App> {
    ServiceLocator::desktop()
        .get_pinned_apps()
        .map(|app| {
            let icon = app
                .icon_path()
                .map(|icon_path| icon_path.to_string_lossy().to_string())
                .unwrap_or_default();
            ffi::App {
                id: app.id().clone(),
                name: app.name().clone(),
                icon,
            }
        })
        .collect()
}

fn launch_app(app_id: &str) -> bool {
    ServiceLocator::desktop().launch_app(app_id).is_some()
}
