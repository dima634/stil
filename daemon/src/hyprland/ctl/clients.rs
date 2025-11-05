use super::HyprCtlCmd;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Workspace {
    pub id: u32,
    pub name: String,
}

impl Into<proto_rust::Workspace> for Workspace {
    fn into(self) -> proto_rust::Workspace {
        let mut workspace_dto = proto_rust::Workspace::default();
        workspace_dto.set_id(self.id);
        workspace_dto.set_name(self.name);
        workspace_dto
    }
}

#[derive(Debug, Deserialize)]
pub struct Client {
    pub address: String,
    pub class: String,
    pub workspace: Workspace,
}

impl Into<proto_rust::Client> for Client {
    fn into(self) -> proto_rust::Client {
        let workspace_dto: proto_rust::Workspace = self.workspace.into();
        let mut client_dto = proto_rust::Client::default();
        client_dto.set_class(self.class);
        client_dto.set_address(self.address);
        client_dto.set_workspace(workspace_dto);

        client_dto
    }
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

impl Into<proto_rust::Clients> for Clients {
    fn into(self) -> proto_rust::Clients {
        let vec = self.0;
        let client_dtos = vec.into_iter().map(|client| Into::<proto_rust::Client>::into(client));

        let mut clients_dto = proto_rust::Clients::default();
        clients_dto.set_clients(client_dtos);
        clients_dto
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
