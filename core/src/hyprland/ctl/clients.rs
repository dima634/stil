use super::HyprCtlCmd;
use serde::Deserialize;

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
struct ClientDto {
    address: String,
    class: String,
    workspace: ClientWorkspace,
}

#[derive(Debug, Deserialize)]
#[serde(try_from = "ClientDto")]
pub struct Client {
    address: usize,
    class: String,
    workspace: ClientWorkspace,
}

impl TryFrom<ClientDto> for Client {
    type Error = &'static str;

    fn try_from(value: ClientDto) -> Result<Self, Self::Error> {
        Ok(Client {
            address: usize::from_str_radix(&value.address[2..], 16).map_err(|_| "invalid address format")?,
            class: value.class,
            workspace: value.workspace,
        })
    }
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
    pub fn address(&self) -> usize {
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
        serde_json::from_str(value).map_err(|_| ()).map(|vec| Self(vec))
    }
}

#[derive(Debug)]
pub struct GetClientsCmd;

impl ToString for GetClientsCmd {
    #[inline]
    fn to_string(&self) -> String {
        "-j/clients".to_string()
    }
}

impl HyprCtlCmd for GetClientsCmd {
    type Response<'str> = Clients;
}
