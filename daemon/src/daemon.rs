use super::hyprland::{GetClientsCmd, HyprCtl};
use protobuf::Serialize;
use std::{fs, io::Write, os::unix::net::UnixListener, path::PathBuf};
use tracing::{error, info};

pub fn start_daemon() {
    info!("Starting Stil daemon...");

    let socket_path = window_events_socket_path();
    if socket_path.exists() {
        let Ok(_) = fs::remove_file(&socket_path) else {
            error!("Failed to remove existing socket file");
            return;
        };
    }

    let Ok(listener) = UnixListener::bind(&socket_path) else {
        error!("Failed to bind to socket");
        return;
    };

    info!("Daemon initialized and listening on {:?}", socket_path);

    loop {
        let Ok((mut stream, addr)) = listener.accept() else {
            error!("Failed to accept incoming connection");
            continue;
        };

        info!("New connection from {:?}. Sending current state...", addr);
        let clients = get_current_desktop_state();
        let Ok(bytes) = clients.serialize() else {
            error!("Failed to serialize current desktop state");
            continue;
        };

        if stream.write_all(&bytes).is_err() {
            error!("Failed to send current desktop state");
        };
    }
}

fn get_current_desktop_state() -> proto_rust::Clients {
    let mut clients_dto = proto_rust::Clients::default();
    let mut hyprctl = HyprCtl::default();
    let Some(clients) = hyprctl.run(GetClientsCmd) else {
        error!("Failed to get clients from Hyprland");
        return clients_dto;
    };

    let client_dtos = clients.0.into_iter().map(|client| {
        let mut workspace_dto = proto_rust::Workspace::default();
        workspace_dto.set_id(client.workspace.id);
        workspace_dto.set_name(client.workspace.name);

        let mut client_dto = proto_rust::Client::default();
        client_dto.set_class(client.class);
        client_dto.set_address(client.address);
        client_dto.set_workspace(workspace_dto);

        client_dto
    });

    clients_dto.set_clients(client_dtos);
    clients_dto
}

fn window_events_socket_path() -> PathBuf {
    std::env::temp_dir().join("stil.sock")
}
