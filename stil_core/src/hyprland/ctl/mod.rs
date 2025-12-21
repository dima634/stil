mod active_window;
mod active_workspace;
mod clients;
mod devices;
mod exec;
mod workspaces;

pub use active_window::*;
pub use active_workspace::*;
pub use clients::*;
pub use devices::*;
pub use exec::*;
pub use workspaces::*;

use super::Hyprland;
use std::{
    fmt::Display,
    io::{Read, Write},
    os::unix::net::UnixStream,
};

pub trait HyprCtlCmd: ToString {
    type Response<'str>: TryFrom<&'str str>;
}

#[derive(Debug)]
pub struct EmptyResponse;

impl Display for EmptyResponse {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl TryFrom<&str> for EmptyResponse {
    type Error = ();

    #[inline]
    fn try_from(_: &str) -> Result<Self, Self::Error> {
        Ok(EmptyResponse)
    }
}

#[derive(Debug, Default)]
pub struct HyprCtl {
    read_buffer: Vec<u8>,
}

impl HyprCtl {
    // TODO: batching
    pub fn run<T: HyprCtlCmd>(&mut self, cmd: T) -> Option<T::Response<'_>> {
        let socket_path = Hyprland::ctl_socket_path()?;
        let mut socket = UnixStream::connect(socket_path).ok()?;

        let cmd_str = cmd.to_string();
        socket.write_all(cmd_str.as_bytes()).ok()?;

        self.read_buffer.clear();
        let num_bytes = socket.read_to_end(&mut self.read_buffer).ok()?;
        let raw_response = std::str::from_utf8(&self.read_buffer[..num_bytes]).ok()?;

        T::Response::try_from(raw_response).ok()
    }
}
