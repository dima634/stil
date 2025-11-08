use super::{Client, HyprCtlCmd};

#[derive(Debug)]
pub struct GetActiveWindowCmd;

impl ToString for GetActiveWindowCmd {
    #[inline]
    fn to_string(&self) -> String {
        "-j/activewindow".to_string()
    }
}

impl HyprCtlCmd for GetActiveWindowCmd {
    type Response<'str> = Client;
}
