use super::HyprCtlCmd;
use serde::Deserialize;
use std::fmt::Display;

#[derive(Debug, Deserialize)]
pub struct Workspace {
    pub id: i32,
    pub name: String,
    #[serde(alias = "ispersistent")]
    pub is_persistent: bool,
}

impl TryFrom<&str> for Workspace {
    type Error = ();

    #[inline]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(value).map_err(|_| ())
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct Workspaces(pub Vec<Workspace>);

impl TryFrom<&str> for Workspaces {
    type Error = ();

    #[inline]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(value).map_err(|_| ()).map(Self)
    }
}

#[derive(Debug)]
pub struct GetWorkspacesCmd;

impl Display for GetWorkspacesCmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "-j/workspaces")
    }
}

impl HyprCtlCmd for GetWorkspacesCmd {
    type Response<'str> = Workspaces;
}
