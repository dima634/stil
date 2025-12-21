use super::{HyprCtlCmd, Workspace};
use std::fmt::Display;

#[derive(Debug)]
pub struct GetActiveWorkspaceCmd;

impl Display for GetActiveWorkspaceCmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "-j/activeworkspace")
    }
}

impl HyprCtlCmd for GetActiveWorkspaceCmd {
    type Response<'str> = Workspace;
}
