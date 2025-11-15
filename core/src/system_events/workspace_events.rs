#[cxx::bridge(namespace = "core")]
mod ffi {
    #[derive(Debug, Clone)]
    pub struct WorkspaceCreated {
        pub id: i32,
        pub name: String,
    }
}

pub use ffi::WorkspaceCreated;
