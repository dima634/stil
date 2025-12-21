use super::HyprCtlCmd;
use crate::hyprland;
use serde::Deserialize;
use std::fmt::Display;

#[derive(Debug, Deserialize)]
struct ClientWorkspace {
    id: i32,
    name: String,
}

impl ClientWorkspace {
    #[inline]
    pub fn id(&self) -> i32 {
        self.id
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Deserialize)]
pub struct Client {
    address: hyprland::Address,
    class: String,
    workspace: ClientWorkspace,
}

impl Client {
    #[inline]
    pub fn workspace(&self) -> i32 {
        self.workspace.id()
    }

    #[inline]
    pub fn workspace_name(&self) -> &str {
        self.workspace.name()
    }

    #[inline]
    pub fn class(&self) -> &str {
        &self.class
    }

    #[inline]
    pub fn address(&self) -> hyprland::Address {
        self.address
    }
}

impl TryFrom<&str> for Client {
    type Error = ();

    #[inline]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(value).map_err(|_| ())
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct Clients(pub Vec<Client>);

impl TryFrom<&str> for Clients {
    type Error = ();

    #[inline]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(value).map_err(|_| ()).map(Self)
    }
}

#[derive(Debug)]
pub struct GetClientsCmd;

impl Display for GetClientsCmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "-j/clients")
    }
}

impl HyprCtlCmd for GetClientsCmd {
    type Response<'str> = Clients;
}
