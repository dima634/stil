use crate::{
    application::ApplicationService,
    hyprland,
    system_events::{SystemEvent, WindowClosed, WindowMoved, WindowOpened, WorkspaceCreated},
};
use std::sync::{RwLock, atomic::AtomicI32, mpsc::Sender};
use tracing::warn;

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
        let active_window_addr = hypr_ctl.run(hyprland::GetActiveWindowCmd).map(|w| w.address());
        let hyprland::Clients(clients) = hypr_ctl.run(hyprland::GetClientsCmd).unwrap_or_default();
        let windows = clients
            .into_iter()
            .map(|client| Window {
                address: client.address(),
                app_id: app_service
                    .find_app_by_wmclass(client.class())
                    .map(|app| app.id().clone()),
                workspace_id: client.workspace(),
                class: client.class().to_string(),
                is_focused: active_window_addr == Some(client.address()),
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

    pub fn get_workspace_id_by_name(&self, name: &str) -> Option<i32> {
        let workspaces = self.workspaces.read().unwrap();
        workspaces.iter().find(|ws| ws.name == name).map(|ws| ws.id)
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
impl WorkspaceService {
    pub fn get_window_by_address(&self, address: hyprland::Address) -> Option<Window> {
        let windows = self.windows.read().unwrap();
        windows.iter().find(|w| w.address == address).cloned()
    }

    pub fn add_window(&self, address: hyprland::Address, app_id: Option<String>, workspace_id: i32, class: String) {
        let mut windows = self.windows.write().unwrap();
        if windows.iter().any(|w| w.address == address) {
            warn!("Window with address {} already exists, skipping add", address);
            return;
        }

        let window = Window {
            address,
            app_id,
            workspace_id,
            class,
            is_focused: false,
        };

        let event = WindowOpened {
            address: window.address,
            workspace_id: window.workspace_id,
            app_id: window.app_id.clone(),
        };
        windows.push(window);
        let _ = self.event_sender.send(SystemEvent::WindowOpened(event));
    }

    pub fn remove_window(&self, address: hyprland::Address) {
        let mut windows = self.windows.write().unwrap();
        let Some(window_idx) = windows.iter().position(|w| w.address == address) else {
            warn!("Tried to remove non-existing window with address {}", address);
            return;
        };

        let workspace_id = windows[window_idx].workspace_id;
        windows.swap_remove(window_idx);
        let _ = self
            .event_sender
            .send(SystemEvent::WindowClosed(WindowClosed { address, workspace_id }));
    }

    pub fn set_focused_window(&self, window_address: hyprland::Address) {
        let mut windows = self.windows.write().unwrap();
        for window in windows.iter_mut() {
            window.is_focused = window.address == window_address;
        }
        let _ = self.event_sender.send(SystemEvent::WindowFocused(window_address));
    }

    pub fn move_window_to_workspace(&self, window_address: hyprland::Address, target_workspace_id: i32) {
        let mut windows = self.windows.write().unwrap();
        let Some(window) = windows.iter_mut().find(|w| w.address == window_address) else {
            warn!("Tried to move non-existing window with address {}", window_address);
            return;
        };

        if window.workspace_id == target_workspace_id {
            return;
        }

        let event = WindowMoved {
            window_address,
            from_workspace: window.workspace_id,
            to_workspace: target_workspace_id,
        };
        window.workspace_id = target_workspace_id;
        let _ = self.event_sender.send(SystemEvent::WindowMoved(event));
    }
}
