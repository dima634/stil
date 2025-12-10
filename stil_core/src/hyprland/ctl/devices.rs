use super::HyprCtlCmd;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Keyboard {
    pub name: String,
    pub active_keymap: String,
    pub main: bool,
    #[serde(rename = "capsLock")]
    pub caps_lock: bool,
    #[serde(rename = "numLock")]
    pub num_lock: bool,
}

#[derive(Debug, Deserialize)]
pub struct Devices {
    pub keyboards: Vec<Keyboard>,
}

impl TryFrom<&str> for Devices {
    type Error = serde_json::Error;

    #[inline]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(value)
    }
}

#[derive(Debug)]
pub struct GetDevicesCmd;

impl ToString for GetDevicesCmd {
    #[inline]
    fn to_string(&self) -> String {
        "-j/devices".to_string()
    }
}

impl HyprCtlCmd for GetDevicesCmd {
    type Response<'a> = Devices;
}
