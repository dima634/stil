use super::Workspace;
use crate::hyprland::{GetWorkspacesCmd, HyprCtl, Workspaces};

pub struct WorkspaceRegistry {
    workspaces: Vec<Workspace>,
}

impl WorkspaceRegistry {
    pub fn workspaces(&self) -> &[Workspace] {
        &self.workspaces
    }
}

impl Default for WorkspaceRegistry {
    fn default() -> Self {
        let Workspaces(hypr_clients) = HyprCtl::default().run(GetWorkspacesCmd).unwrap_or_default();
        let workspaces = hypr_clients
            .into_iter()
            .map(|workspace| Workspace::new(workspace.id, workspace.name))
            .collect();

        Self { workspaces }
    }
}

lazy_static::lazy_static! {
    static ref WORKSPACE_REGISTRY: WorkspaceRegistry = WorkspaceRegistry::default();
}

pub fn get_workspace_registry() -> &'static WorkspaceRegistry {
    &WORKSPACE_REGISTRY
}
