use crate::{freedesktop, hyprland, repos, system_events};
use tracing::warn;

#[derive(Debug, Default)]
pub struct Workspace {
    id: i32,
    name: String,
}

impl Workspace {
    #[inline]
    pub fn id(&self) -> i32 {
        self.id
    }

    #[inline]
    pub fn name(&self) -> &String {
        &self.name
    }
}

#[derive(Debug)]
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
pub struct App {
    desktop_entry: freedesktop::DesktopEntry,
    icon_path: Option<std::path::PathBuf>,
    is_pinned: bool,
}

impl App {
    #[inline]
    pub fn icon_path(&self) -> Option<&std::path::PathBuf> {
        self.icon_path.as_ref()
    }

    #[inline]
    pub fn is_pinned(&self) -> bool {
        self.is_pinned
    }
}

#[derive(Debug, Default)]
pub struct Desktop {
    current_workspace_id: i32,
    workspaces: WorkspaceVec,
    windows: WindowVec,
    applications: Vec<App>,
}

impl Desktop {
    pub fn init(&mut self) {
        let pinned_apps = repos::ApplicationRepo::default().get_pinned();
        let desktop_entries = freedesktop::find_application_desktop_entries();
        let mut icon_lookup = freedesktop::IconLookup::default();
        let apps = desktop_entries
            .into_iter()
            .map(|desktop_entry| App {
                is_pinned: pinned_apps.iter().any(|app| app.id == desktop_entry.id),
                icon_path: desktop_entry
                    .icon
                    .as_ref()
                    .map(|icon| icon_lookup.find_icon(icon, 48, 1))
                    .flatten(),
                desktop_entry,
            })
            .collect();
        self.applications = apps;

        let mut hypr_ctl = hyprland::HyprCtl::default();
        let hyprland::Clients(clients) = hypr_ctl.run(hyprland::GetClientsCmd).unwrap_or_default();
        let windows = clients
            .into_iter()
            .map(|client| Window {
                address: hyprland::Address(client.address()),
                app_id: self
                    .find_app_by_wmclass(client.class())
                    .map(|app| app.desktop_entry.id.clone()),
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

        self.current_workspace_id = hypr_ctl
            .run(hyprland::GetActiveWorkspaceCmd)
            .map(|ws| ws.id)
            .unwrap_or(i32::MAX);
        self.windows = WindowVec(windows);
        self.workspaces = WorkspaceVec(workspaces);
    }

    fn find_app_by_wmclass(&self, wm_class: &str) -> Option<&App> {
        self.applications
            .iter()
            .find(|app| app.desktop_entry.wm_class.as_deref() == Some(wm_class))
            .or_else(|| self.applications.iter().find(|app| app.desktop_entry.id == wm_class))
    }
}

#[derive(Debug, Default)]
struct WorkspaceVec(Vec<Workspace>);

impl WorkspaceVec {
    fn get_by_id(&self, id: i32) -> Option<&Workspace> {
        self.0.iter().find(|ws| ws.id == id)
    }

    fn get_by_name(&self, name: &str) -> Option<&Workspace> {
        self.0.iter().find(|ws| ws.name == name)
    }
}

#[derive(Debug, Default)]
struct WindowVec(Vec<Window>);

impl WindowVec {
    fn get_by_workspace_id(&self, workspace_id: i32) -> impl Iterator<Item = &Window> {
        self.0.iter().filter(move |w| w.workspace_id == workspace_id)
    }

    fn get_by_address_mut(&mut self, address: hyprland::Address) -> Option<&mut Window> {
        self.0.iter_mut().find(|w| w.address == address)
    }
}

// Workspace APIs
impl Desktop {
    #[inline]
    pub fn get_workspaces(&self) -> impl Iterator<Item = &Workspace> {
        self.workspaces.0.iter()
    }

    #[inline]
    pub fn get_workspace_windows(&self, workspace_id: i32) -> impl Iterator<Item = &Window> {
        self.windows.get_by_workspace_id(workspace_id)
    }

    #[inline]
    pub fn get_current_workspace_id(&self) -> i32 {
        self.current_workspace_id
    }

    pub fn get_current_workspace(&self) -> Option<&Workspace> {
        self.workspaces.0.iter().find(|ws| ws.id == self.current_workspace_id)
    }

    pub fn remove_workspace(&mut self, workspace_id: i32) {
        self.workspaces.0.retain(|ws| ws.id != workspace_id);
        self.windows.0.retain(|w| w.workspace_id != workspace_id);
    }
}

// Window APIs
impl Desktop {
    pub fn get_window(&self, address: hyprland::Address) -> Option<&Window> {
        self.windows.0.iter().find(|w| w.address == address)
    }

    pub fn close_window(&mut self, address: hyprland::Address) {
        self.windows.0.retain(|w| w.address != address);
    }

    pub fn move_window_to_workspace(&mut self, window_address: hyprland::Address, workspace_id: i32) {
        let Some(window) = self.windows.get_by_address_mut(window_address) else {
            warn!("Tried to move non-existent window {}", window_address);
            return;
        };

        if self.workspaces.get_by_id(workspace_id).is_none() {
            warn!(
                "Tried to move window {} to non-existent workspace {}",
                window_address, workspace_id
            );
            return;
        };

        window.workspace_id = workspace_id;
    }
}

// Application APIs
impl Desktop {
    #[inline]
    pub fn get_app(&self, app_id: &str) -> Option<&App> {
        self.applications.iter().find(|app| app.desktop_entry.id == app_id)
    }
}

impl Desktop {
    pub fn consume_system_event(&mut self, event: &system_events::SystemEvent) {
        use system_events::SystemEvent::*;

        match event {
            WorkspaceCreated(workspace_created) => {
                self.workspaces.0.push(Workspace {
                    id: workspace_created.id,
                    name: workspace_created.name.clone(),
                });
            }
            WorkspaceDestroyed(id) => self.remove_workspace(*id),
            WorkspaceFocused(id) => self.current_workspace_id = *id,
            WindowOpened(window_opened) => {
                let Some(ws) = self.workspaces.get_by_name(&window_opened.workspace_name) else {
                    warn!(
                        "Received WindowOpened event for workspace '{}' which does not exist",
                        window_opened.workspace_name
                    );
                    return;
                };

                self.windows.0.push(Window {
                    address: hyprland::Address(window_opened.address),
                    app_id: self
                        .find_app_by_wmclass(&window_opened.class_name)
                        .map(|app| app.desktop_entry.id.clone()),
                    workspace_id: ws.id,
                    class: window_opened.class_name.clone(),
                    is_focused: false,
                });
            }
            WindowClosed(address) => self.close_window(hyprland::Address(*address)),
            WindowFocused(address) => {
                if let Some(window) = self.windows.get_by_address_mut(hyprland::Address(*address)) {
                    window.is_focused = true;
                }
            }
            WindowMoved(window_moved) => {
                self.move_window_to_workspace(hyprland::Address(window_moved.address), window_moved.workspace_id)
            }
            Empty => {}
        }
    }
}
