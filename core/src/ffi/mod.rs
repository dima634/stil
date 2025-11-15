use crate::{
    services::global_init,
    workspaces::{Workspace, WorkspacesState, get_workspaces_state as get_workspaces_state_rs},
};

mod system_events;
mod hyprland;

#[cxx::bridge(namespace = "core")]
mod ffi {
    extern "Rust" {
        type WorkspacesState;

        fn current_workspace_id(&self) -> i32;
        fn workspaces(&self) -> &Vec<Workspace>;
    }

    extern "Rust" {
        type Workspace;

        fn id(&self) -> i32;
        fn name(&self) -> &String;
        fn get_workspaces_state() -> Box<WorkspacesState>;
    }
}

fn get_workspaces_state() -> Box<WorkspacesState> {
    global_init();
    Box::new(get_workspaces_state_rs())
}
