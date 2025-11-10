use super::HyprCtlCmd;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ClientWorkspace {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Client {
    pub address: String,
    pub class: String,
    pub workspace: ClientWorkspace,
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
