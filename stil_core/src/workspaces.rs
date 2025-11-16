use super::hyprland::{GetActiveWorkspaceCmd, GetWorkspacesCmd, HyprCtl, Workspaces};

#[derive(Debug)]
pub struct Workspace {
    id: i32,
    name: String,
}

impl Workspace {
    pub fn new(id: i32, name: String) -> Self {
        Workspace { id, name }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

impl From<&super::hyprland::WorkspaceV2> for Workspace {
    fn from(ws: &super::hyprland::WorkspaceV2) -> Self {
        Workspace {
            id: ws.id,
            name: ws.name.clone(),
        }
    }
}

pub struct WorkspacesState {
    current_workspace_id: i32,
    workspaces: Vec<Workspace>,
}

impl WorkspacesState {
    #[inline]
    pub fn current_workspace_id(&self) -> i32 {
        self.current_workspace_id
    }

    #[inline]
    pub fn workspaces(&self) -> &Vec<Workspace> {
        &self.workspaces
    }
}

/// Returns the list of workspaces and the ID of the currently active workspace
pub fn get_workspaces_state() -> WorkspacesState {
    let mut hypr_ctl = HyprCtl::default();
    let Workspaces(hypr_clients) = hypr_ctl.run(GetWorkspacesCmd).unwrap_or_default();
    let current_workspace_id = hypr_ctl
        .run(GetActiveWorkspaceCmd)
        .map(|active_ws| active_ws.id)
        .unwrap_or(0);
    let workspaces = hypr_clients
        .into_iter()
        .map(|workspace| Workspace::new(workspace.id, workspace.name))
        .collect();

    WorkspacesState {
        current_workspace_id,
        workspaces,
    }
}
