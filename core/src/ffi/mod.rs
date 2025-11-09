use crate::workspaces::{Workspace, WorkspaceRegistry, get_workspace_registry};

#[cxx::bridge]
mod workspaces {
    extern "Rust" {
        type Workspace;

        fn id(&self) -> i32;
        fn name(&self) -> &String;
    }

    extern "Rust" {
        type WorkspaceRegistry;

        fn workspaces(&self) -> &[Workspace];
        fn get_workspace_registry() -> &'static WorkspaceRegistry;
    }
}
