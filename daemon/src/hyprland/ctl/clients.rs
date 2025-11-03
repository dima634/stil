use super::HyprCtlCmd;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Workspace {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Client {
    pub address: String,
    pub class: String,
    pub workspace: Workspace,
}

#[derive(Debug, Deserialize)]
pub struct Clients(pub Vec<Client>);

impl TryFrom<&str> for Clients {
    type Error = ();

    #[inline]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(value).map_err(|_| ()).map(|vec| Self(vec))
    }
}

#[derive(Debug)]
pub struct ClientsCmd;

impl ToString for ClientsCmd {
    #[inline]
    fn to_string(&self) -> String {
        "-j/clients".to_string()
    }
}

impl HyprCtlCmd for ClientsCmd {
    type Response<'str> = Clients;
}
