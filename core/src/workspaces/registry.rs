use super::Workspace;
use crate::{
    hyprland::{self, GetActiveWorkspaceCmd, GetWorkspacesCmd, HyprCtl, Workspaces},
    system_events::{SystemEvent, SystemEventConsumer},
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

impl SystemEventConsumer for WorkspaceRegistry {
    fn consume(&mut self, event: &SystemEvent) {
        use hyprland::Event as HyprEvent;

        match event {
            SystemEvent::Hyprland(HyprEvent::ActiveWindow(_)) => {}
            SystemEvent::Hyprland(HyprEvent::CloseWindow(_)) => {}
            SystemEvent::Hyprland(HyprEvent::OpenWindow(_)) => {}
            SystemEvent::Hyprland(HyprEvent::CreateWorkspace(hyprland::CreateWorkspaceV2(workspace))) => {
                self.workspaces.push(Workspace::new(workspace.id, workspace.name.clone()));
            }
            SystemEvent::Hyprland(HyprEvent::DestroyWorkspace(hyprland::DestroyWorkspaceV2(workspace))) => {
                let idx = self.workspaces.iter().position(|w| w.id() == workspace.id);
                if let Some(idx) = idx {
                    self.workspaces.swap_remove(idx);
                }
            }
        }
    }
}
