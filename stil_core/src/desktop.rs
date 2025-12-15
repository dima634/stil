use crate::{
    application::ApplicationService,
    db,
    keyboard::KeyboardService,
    workspace::{Window, Workspace, WorkspaceService},
};
use std::sync::Once;
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
    pub fn new() -> Self {
        static INIT: Once = Once::new();
        INIT.call_once(|| {
            if let Err(e) = configure_logging() {
                eprintln!("Failed to configure logging. Inner error: {}", e);
            }

            if let Err(e) = db::migrate_up() {
                error!("Failed to migrate database up. Inner error: {}", e);
            }

            info!("Desktop initialized");
        });

        let keyboard_service = KeyboardService::default();
        let application_service = ApplicationService::default();
        let workspace_service = WorkspaceService::new(&application_service);

        Self {
            workspace_service,
            application_service,
            keyboard_service,
        }
    }
}

// Workspace API
impl Desktop {
    #[inline]
    pub fn get_current_workspace_id(&self) -> i32 {
        self.workspace_service.get_current_workspace_id()
    }

    #[inline]
    pub fn get_workspaces(&self) -> impl Iterator<Item = &Workspace> {
        self.workspace_service.get_workspaces()
    }
}

// Window API
impl Desktop {
    #[inline]
    pub fn get_workspace_windows(&self, workspace_id: i32) -> impl Iterator<Item = &Window> {
        self.workspace_service.get_workspace_windows(workspace_id)
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
