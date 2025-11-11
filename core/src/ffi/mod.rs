use std::sync::MutexGuard;
use crate::{services::workspace_registry, workspaces::{Workspace, WorkspaceRegistry}};

#[cxx::bridge]
mod workspaces {
    extern "Rust" {
        type Workspace;

        fn id(&self) -> i32;
        fn name(&self) -> &String;
    }

    extern "Rust" {
        type Workspaces;

        #[Self = "Workspaces"]
        fn lock() -> Box<Workspaces>;
        fn all(&self) -> &[Workspace];
        fn current_workspace_id(&self) -> i32;
    }
}

struct Workspaces(MutexGuard<'static, WorkspaceRegistry>);

impl Workspaces {
    pub fn lock() -> Box<Self> {
        Box::new(Self(workspace_registry()))
    }

    pub fn all(&self) -> &[Workspace] {
        self.0.workspaces()
    }

    pub fn current_workspace_id(&self) -> i32 {
        self.0.current_workspace_id()
    }
}
