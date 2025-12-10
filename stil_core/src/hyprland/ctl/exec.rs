use super::{EmptyResponse, HyprCtlCmd};

#[derive(Debug)]
pub struct ExecCmd(String);

impl ExecCmd {
    #[inline]
    pub fn new(cmd: String) -> Self {
        Self(cmd)
    }
}

impl ToString for ExecCmd {
    fn to_string(&self) -> String {
        format!("dispatch exec {}", self.0)
    }
}

impl HyprCtlCmd for ExecCmd {
    type Response<'a> = EmptyResponse;
}
