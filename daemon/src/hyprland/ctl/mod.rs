mod clients;

pub use clients::{Clients, GetClientsCmd};

use super::Hyprland;
use std::{
    io::{Read, Write},
    os::unix::net::UnixStream,
};

pub trait HyprCtlCmd: ToString {
    type Response<'str>: TryFrom<&'str str>;
}

#[derive(Debug, Default)]
pub struct HyprCtl {
    read_buffer: Vec<u8>,
}

impl HyprCtl {
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
