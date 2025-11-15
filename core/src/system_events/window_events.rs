#[cxx::bridge(namespace = "core")]
mod ffi {
    #[derive(Debug, Clone)]
    pub struct WindowOpened {
        pub address: usize,
        pub workspace: i32,
        pub class: String,
    }
}

pub use ffi::WindowOpened;
