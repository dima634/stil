use crate::{
    application::{App, ApplicationService},
    db, hyprland,
    keyboard::KeyboardService,
    notifications::NotificationsService,
    system_events::SystemEvent,
    workspace::{Window, Workspace, WorkspaceService},
};
use std::{
    ops::ControlFlow,
    sync::{Arc, Once, RwLock, mpsc::Receiver},
};
use tracing::{error, info, warn};

/// Desktop facade
#[derive(Debug)]
pub struct Desktop {
    workspace_service: WorkspaceService,
    application_service: ApplicationService,
    keyboard_service: RwLock<KeyboardService>,
    notifications_service: Option<NotificationsService>,
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

        let keyboard_service = RwLock::new(KeyboardService::new(system_event_tx.clone()));
        let application_service = ApplicationService::default();
        let workspace_service = WorkspaceService::new(&application_service, system_event_tx.clone());
        let notifications_service = NotificationsService::new(system_event_tx.clone());
        let desktop = Arc::new(Self {
            workspace_service,
            application_service,
            keyboard_service,
            notifications_service,
        });

        let desktop_clone = desktop.clone();
        if let Err(e) = std::thread::Builder::new()
            .name("hypr_ev".to_string())
            .spawn(move || listen_for_hyprland_events(desktop_clone))
        {
            error!("Failed to spawn Hyprland event listener thread. Inner error: {}", e);
        }

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
    #[inline]
    pub fn get_workspace_windows(&self, workspace_id: i32) -> Vec<Window> {
        self.workspace_service.get_workspace_windows(workspace_id)
    }

    #[inline]
    pub fn get_window_by_address(&self, address: hyprland::Address) -> Option<Window> {
        self.workspace_service.get_window_by_address(address)
    }

    fn add_window(&self, address: hyprland::Address, wm_class: String, workspace_name: String) {
        let app_id = self
            .application_service
            .find_app_by_wmclass(&wm_class)
            .map(|app| app.id().clone());
        let Some(workspace_id) = self.workspace_service.get_workspace_id_by_name(&workspace_name) else {
            warn!("Could not find workspace id with name: {}", workspace_name);
            return;
        };
        self.workspace_service
            .add_window(address, app_id, workspace_id, wm_class);
    }
}

// App API
impl Desktop {
    #[inline]
    pub fn get_app(&self, app_id: &str) -> Option<&App> {
        self.application_service.get_app_by_id(app_id)
    }
}

// Keyboard
impl Desktop {
    pub fn get_current_keyboard_layout_code(&self) -> Option<&'static str> {
        self.keyboard_service
            .read()
            .unwrap()
            .get_main_keyboard()
            .map(|kb| kb.active_layout_code())
    }
}

fn configure_logging() -> Result<(), tracing::subscriber::SetGlobalDefaultError> {
    use tracing::Level;
    use tracing_subscriber::FmtSubscriber;

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_thread_names(true)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
}

fn listen_for_hyprland_events(desktop: Arc<Desktop>) {
    info!("Starting Hyprland event listener");
    let handle_event = |event| {
        match event {
            hyprland::Event::CreateWorkspace(workspace) => {
                desktop
                    .workspace_service
                    .add_workspace(Workspace::new(workspace.id, workspace.name));
            }
            hyprland::Event::DestroyWorkspace(workspace) => {
                desktop.workspace_service.remove_workspace(workspace.id);
            }
            hyprland::Event::FocusWorkspace(workspace) => {
                desktop.workspace_service.set_current_workspace(workspace.id);
            }
            hyprland::Event::OpenWindow(open_window) => {
                desktop.add_window(
                    open_window.window_address,
                    open_window.window_class,
                    open_window.workspace_name,
                );
            }
            hyprland::Event::CloseWindow(close_window) => {
                desktop.workspace_service.remove_window(close_window.window_address);
            }
            hyprland::Event::ActiveWindowV2(active_window) => {
                desktop.workspace_service.set_focused_window(active_window.address);
            }
            hyprland::Event::MoveWindowV2(move_window) => {
                desktop
                    .workspace_service
                    .move_window_to_workspace(move_window.window_address, move_window.workspace_id);
            }
            hyprland::Event::ActiveLayout(layout) => {
                desktop
                    .keyboard_service
                    .write()
                    .unwrap()
                    .set_keyboard_keymap(&layout.keyboard_name, layout.layout_name);
            }
        };
        ControlFlow::Continue(())
    };

    if hyprland::HyprEvents::listen(handle_event).is_none() {
        error!("Failed to start Hyprland event listener");
    }
}
