use crate::{
    application::{App, ApplicationService},
    db, hyprland,
    keyboard::KeyboardService,
    system_events::SystemEvent,
    workspace::{Window, Workspace, WorkspaceService},
};
use std::{
    ops::ControlFlow,
    sync::{Arc, Once, mpsc::Receiver},
};
use tracing::{error, info};

/// Desktop facade
#[derive(Debug)]
pub struct Desktop {
    workspace_service: WorkspaceService,
    application_service: ApplicationService,
    keyboard_service: KeyboardService,
}

// Initialization
impl Desktop {
    pub fn new() -> (Arc<Self>, Receiver<SystemEvent>) {
        static INIT: Once = Once::new();
        INIT.call_once(|| {
            if let Err(e) = configure_logging() {
                eprintln!("Failed to configure logging. Inner error: {}", e);
            }

            if let Err(e) = db::migrate_up() {
                error!("Failed to migrate database up. Inner error: {}", e);
            }

            info!("Database and logging initialized");
        });

        let (system_event_tx, system_event_rx) = std::sync::mpsc::channel();

        let keyboard_service = KeyboardService::default();
        let application_service = ApplicationService::default();
        let workspace_service = WorkspaceService::new(&application_service, system_event_tx.clone());
        let desktop = Arc::new(Self {
            workspace_service,
            application_service,
            keyboard_service,
        });

        let desktop_listener = desktop.clone();
        std::thread::spawn(move || {
            info!("Starting Hyprland event listener");
            let handle_event = |event| {
                match event {
                    hyprland::Event::CreateWorkspace(workspace) => {
                        desktop_listener
                            .workspace_service
                            .add_workspace(Workspace::new(workspace.id, workspace.name));
                    }
                    hyprland::Event::DestroyWorkspace(workspace) => {
                        desktop_listener.workspace_service.remove_workspace(workspace.id)
                    }
                    hyprland::Event::FocusWorkspace(workspace) => {
                        desktop_listener.workspace_service.set_current_workspace(workspace.id)
                    }
                    hyprland::Event::OpenWindow(_) => {}
                    hyprland::Event::CloseWindow(_) => {}
                    hyprland::Event::ActiveWindowV2(active_window) => {
                        desktop_listener
                            .workspace_service
                            .set_focused_window(active_window.address);
                    }
                    hyprland::Event::MoveWindowV2(_) => {}
                    hyprland::Event::ActiveLayout(_) => {}
                };
                ControlFlow::Continue(())
            };

            if let None = hyprland::HyprEvents::listen(handle_event) {
                error!("Failed to start Hyprland event listener");
            }
        });

        (desktop, system_event_rx)
    }
}

// Workspace API
impl Desktop {
    #[inline]
    pub fn get_current_workspace_id(&self) -> i32 {
        self.workspace_service.get_current_workspace_id()
    }

    #[inline]
    pub fn get_workspaces(&self) -> Vec<Workspace> {
        self.workspace_service.get_workspaces()
    }
}

// Window API
impl Desktop {
    pub fn get_current_workspace_windows(&self) -> Vec<Window> {
        let workspace_id = self.get_current_workspace_id();
        self.workspace_service.get_workspace_windows(workspace_id)
    }
}

// App API
impl Desktop {
    #[inline]
    pub fn get_app(&self, app_id: &str) -> Option<&App> {
        self.application_service.get_app_by_id(app_id)
    }
}

fn configure_logging() -> Result<(), tracing::subscriber::SetGlobalDefaultError> {
    use tracing::Level;
    use tracing_subscriber::FmtSubscriber;

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .with_thread_ids(true)
        .with_thread_names(true)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
}
