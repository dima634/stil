use super::{EmptyResponse, HyprCtlCmd};

#[derive(Debug)]
pub struct Exec(String);

impl Exec {
    #[inline]
    pub fn new(cmd: String) -> Self {
        Self(cmd)
    }
}

impl ToString for Exec {
    fn to_string(&self) -> String {
        format!("dispatch exec {}", self.0)
    }
}

impl HyprCtlCmd for Exec {
    type Response<'a> = EmptyResponse;
}
