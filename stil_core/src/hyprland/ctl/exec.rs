use super::{EmptyResponse, HyprCtlCmd};
use std::fmt::Display;

#[derive(Debug)]
pub struct ExecCmd(String);

impl ExecCmd {
    #[inline]
    pub fn new(cmd: String) -> Self {
        Self(cmd)
    }
}

impl Display for ExecCmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "dispatch exec {}", self.0)
    }
}

impl HyprCtlCmd for ExecCmd {
    type Response<'a> = EmptyResponse;
}
