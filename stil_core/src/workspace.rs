use tracing::warn;

use crate::{
    application::ApplicationService,
    hyprland,
    system_events::{SystemEvent, WorkspaceCreated},
};
use std::sync::{RwLock, atomic::AtomicI32, mpsc::Sender};

#[derive(Debug, Default, Clone)]
pub struct Workspace {
    id: i32,
    name: String,
}

impl Workspace {
    #[inline]
    pub fn new(id: i32, name: String) -> Self {
        Self { id, name }
    }

    #[inline]
    pub fn id(&self) -> i32 {
        self.id
    }

    #[inline]
    pub fn name(&self) -> &String {
        &self.name
    }
}

#[derive(Debug, Clone)]
pub struct Window {
    address: hyprland::Address,
    app_id: Option<String>,
    class: String,
    workspace_id: i32,
    is_focused: bool,
}

impl Window {
    #[inline]
    pub fn address(&self) -> hyprland::Address {
        self.address
    }

    #[inline]
    pub fn app_id(&self) -> Option<&String> {
        self.app_id.as_ref()
    }

    #[inline]
    pub fn workspace_id(&self) -> i32 {
        self.workspace_id
    }

    #[inline]
    pub fn is_focused(&self) -> bool {
        self.is_focused
    }

    #[inline]
    pub fn class(&self) -> &String {
        &self.class
    }
}

#[derive(Debug)]
pub struct WorkspaceService {
    current_workspace_id: AtomicI32,
    workspaces: RwLock<Vec<Workspace>>,
    windows: RwLock<Vec<Window>>,
    event_sender: Sender<SystemEvent>,
}

impl WorkspaceService {
    pub fn new(app_service: &ApplicationService, system_event_sender: Sender<SystemEvent>) -> Self {
        let mut hypr_ctl = hyprland::HyprCtl::default();
        let hyprland::Clients(clients) = hypr_ctl.run(hyprland::GetClientsCmd).unwrap_or_default();
        let windows = clients
            .into_iter()
            .map(|client| Window {
                address: hyprland::Address(client.address()),
                app_id: app_service
                    .find_app_by_wmclass(client.class())
                    .map(|app| app.id().clone()),
                workspace_id: client.workspace(),
                class: client.class().to_string(),
                is_focused: false, // TODO: determine focused state
            })
            .collect();

        let hyprland::Workspaces(workspaces) = hypr_ctl.run(hyprland::GetWorkspacesCmd).unwrap_or_default();
        let workspaces = workspaces
            .into_iter()
            .map(|ws| Workspace {
                id: ws.id,
                name: ws.name,
            })
            .collect();

        let current_workspace_id = hypr_ctl
            .run(hyprland::GetActiveWorkspaceCmd)
            .map(|ws| ws.id)
            .unwrap_or(i32::MAX);
        let windows = RwLock::new(windows);
        let workspaces = RwLock::new(workspaces);
        Self {
            workspaces,
            windows,
            event_sender: system_event_sender,
            current_workspace_id: AtomicI32::new(current_workspace_id),
        }
    }
}

// Workspace APIs
impl WorkspaceService {
    #[inline]
    pub fn get_workspaces(&self) -> Vec<Workspace> {
        self.workspaces.read().unwrap().clone()
    }

    pub fn get_workspace_windows(&self, workspace_id: i32) -> Vec<Window> {
        self.windows
            .read()
            .unwrap()
            .iter()
            .filter(|w| w.workspace_id == workspace_id)
            .cloned()
            .collect()
    }

    #[inline]
    pub fn get_current_workspace_id(&self) -> i32 {
        use std::sync::atomic::Ordering;
        self.current_workspace_id.load(Ordering::Relaxed)
    }

    pub fn set_current_workspace(&self, workspace_id: i32) {
        use std::sync::atomic::Ordering;
        self.current_workspace_id.store(workspace_id, Ordering::Relaxed);
        let _ = self.event_sender.send(SystemEvent::WorkspaceOpened(workspace_id));
    }

    pub fn add_workspace(&self, workspace: Workspace) {
        let mut workspaces = self.workspaces.write().unwrap();

        if workspaces.iter().any(|ws| ws.id == workspace.id) {
            warn!("Workspace with id {} already exists, skipping add", workspace.id);
            return;
        }

        let event = WorkspaceCreated {
            id: workspace.id,
            name: workspace.name.clone(),
        };
        workspaces.push(workspace);
        let _ = self.event_sender.send(SystemEvent::WorkspaceCreated(event));
    }

    pub fn remove_workspace(&self, workspace_id: i32) {
        let mut workspaces = self.workspaces.write().unwrap();
        let count = workspaces.len();
        workspaces.retain(|ws| ws.id != workspace_id);

        // If a workspace was actually removed
        if workspaces.len() != count {
            self.windows.write().unwrap().retain(|w| w.workspace_id != workspace_id);
            let _ = self.event_sender.send(SystemEvent::WorkspaceDestroyed(workspace_id));
        }
    }
}

// Window APIs
impl WorkspaceService {}
