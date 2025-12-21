use super::{Client, HyprCtlCmd};
use std::fmt::Display;

#[derive(Debug)]
pub struct GetActiveWindowCmd;

impl Display for GetActiveWindowCmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "-j/activewindow")
    }
}

impl HyprCtlCmd for GetActiveWindowCmd {
    type Response<'str> = Client;
}
