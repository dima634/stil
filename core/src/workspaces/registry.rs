use super::Workspace;
use crate::hyprland::{GetActiveWorkspaceCmd, GetWorkspacesCmd, HyprCtl, Workspaces};
use std::sync::{
    LazyLock, Mutex, MutexGuard,
    atomic::{AtomicI32, Ordering},
};

pub struct WorkspaceRegistry {
    workspaces: Vec<Workspace>,
    current_workspace_id: i32,
}

impl WorkspaceRegistry {
    #[inline]
    pub fn workspaces(&self) -> &Vec<Workspace> {
        &self.workspaces
    }

    #[inline]
    pub fn current_workspace_id(&self) -> i32 {
        self.current_workspace_id
    }
}

impl Default for WorkspaceRegistry {
    fn default() -> Self {
        let mut hypr_ctl = HyprCtl::default();
        let Workspaces(hypr_clients) = hypr_ctl.run(GetWorkspacesCmd).unwrap_or_default();
        let current_workspace_id = hypr_ctl.run(GetActiveWorkspaceCmd).map(|active_ws| active_ws.id).unwrap_or(0);
        let workspaces = hypr_clients
            .into_iter()
            .map(|workspace| Workspace::new(workspace.id, workspace.name))
            .collect();
        
        Self {
            workspaces,
            current_workspace_id,
        }
    }
}

static WORKSPACE_REGISTRY: LazyLock<WorkspaceRegistry> = LazyLock::new(|| WorkspaceRegistry::default());

pub fn workspace_registry() -> &'static WorkspaceRegistry {
    &WORKSPACE_REGISTRY
}
