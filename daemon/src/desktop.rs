use super::hyprland::{Client, Clients};

struct Desktop {
    pub clients: Clients,
    pub active_window: Client,
}

impl Into<proto_rust::Desktop> for Desktop {
    fn into(self) -> proto_rust::Desktop {
        let clients: proto_rust::Clients = self.clients.into();
        let active_window: proto_rust::Client = self.active_window.into();

        let mut desktop_dto = proto_rust::Desktop::default();
        desktop_dto.set_clients(clients);
        desktop_dto.set_active_window(active_window);

        desktop_dto
    }
}
