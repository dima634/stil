use super::{HyprCtlCmd, Workspace};

#[derive(Debug)]
pub struct GetActiveWorkspaceCmd;

impl ToString for GetActiveWorkspaceCmd {
    #[inline]
    fn to_string(&self) -> String {
        "-j/activeworkspace".to_string()
    }
}

impl HyprCtlCmd for GetActiveWorkspaceCmd {
    type Response<'str> = Workspace;
}
