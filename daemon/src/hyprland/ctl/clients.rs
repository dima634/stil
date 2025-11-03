use crate::hyprland::HyprCtlCmd;

// TODO: parse as json `hyprctl -j clients`
#[derive(Debug, Default)]
pub struct Client {
    pub window_address: String,
    pub class: String,
    pub workspace_id: String,
}

impl Client {
    #[inline]
    fn with_class(mut self, class: &str) -> Self {
        self.class.push_str(class);
        self
    }

    #[inline]
    fn with_workspace_id(mut self, workspace: &str) -> Self {
        self.workspace_id.push_str(workspace);
        self
    }

    #[inline]
    fn with_window_address(mut self, window_address: &str) -> Self {
        self.window_address.push_str(window_address);
        self
    }
}

#[derive(Debug)]
pub struct Clients(pub Vec<Client>);

impl TryFrom<&str> for Clients {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut clients = Vec::new();
        let mut current = None;

        for line in value.lines() {
            let trimmed = line.trim();

            if trimmed.is_empty() {
                if let Some(client) = current.take() {
                    clients.push(client);
                }
                continue;
            }

            if trimmed.starts_with("Window") {
                let mut parts = trimmed.splitn(3, ' ');
                
                // skip "Window"
                if let None = parts.next() {
                    continue;
                }
                
                if let Some(address) = parts.next() {
                    let mut client = Client::default();
                    client.window_address = address.to_string();
                    current = Some(client);
                }
            } else {
                let mut parts = trimmed.splitn(2, ':');
                let Some(key) = parts.next() else { continue };
                let Some(value) = parts.next().map(|v| v.trim()) else { continue };

                current = match key {
                    "class" => current.map(|client| client.with_class(value)),
                    "workspace" => current.map(|client| client.with_workspace_id(value)),
                    "window_address" => current.map(|client| client.with_window_address(value)),
                    _ => continue,
                };
            }
        }

        Ok(Clients(clients))
    }
}

#[derive(Debug)]
pub struct ClientsCmd;

impl ToString for ClientsCmd {
    #[inline]
    fn to_string(&self) -> String {
        "clients".to_string()
    }
}

impl HyprCtlCmd for ClientsCmd {
    type Response<'str> = Clients;
}
