use crate::{
    hyprland::{Client, Clients, GetClientsCmd, HyprCtl},
    services::global_init,
};

#[cxx::bridge(namespace = "core")]
mod ffi {
    extern "Rust" {
        type Client;
        fn address(&self) -> usize;
        #[cxx_name = "class_"]
        fn class(&self) -> &str;
        fn workspace(&self) -> i32;
        fn workspace_name(&self) -> &str;

        fn get_hyprland_clients() -> Vec<Client>;
    }
}

fn get_hyprland_clients() -> Vec<Client> {
    global_init();
    HyprCtl::default().run(GetClientsCmd).unwrap_or(Clients::default()).0
}
