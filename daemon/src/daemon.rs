use super::hyprland::{Event, GetClientsCmd, HyprCtl, HyprEvents};
use std::{
    cell::RefCell,
    fs,
    io::Write,
    os::unix::net::{UnixListener, UnixStream},
    path::PathBuf,
    rc::Rc,
};
use tracing::{error, info, warn};

pub fn start_daemon() {
    info!("Starting Stil daemon...");

    let socket_path = window_events_socket_path();
    if socket_path.exists() {
        let Ok(_) = fs::remove_file(&socket_path) else {
            error!("Failed to remove existing socket file");
            return;
        };
    }

    let Ok(listener) = UnixListener::bind(&socket_path) else {
        error!("Failed to bind to socket");
        return;
    };

    info!("Daemon initialized and listening on {:?}", socket_path);

    loop {
        let Ok((mut stream, addr)) = listener.accept() else {
            error!("Failed to accept incoming connection");
            continue;
        };

        info!("New connection from {:?}. Sending current state...", addr);

        let clients = get_current_desktop_state();
        let msg = Message::new(clients);

        if msg.write_to_stream(&mut stream).is_err() {
            error!("Failed to send current desktop state");
            continue;
        }

        info!("Sending updates to {:?}", addr);

        let cancelled = Rc::new(RefCell::new(false));
        HyprEvents::listen(cancelled.clone(), |event| {
            let sent = match event {
                Event::OpenWindow(event) => write_msg_to_stream::<proto_rust::OpenWindowEvent, _>(event, &mut stream),
                Event::CloseWindow(event) => write_msg_to_stream::<proto_rust::CloseWindowEvent, _>(event, &mut stream),
                _ => Ok(()),
            };

            match sent {
                Ok(_) => {}
                Err(SendMessageErr::Malformed) => error!("Failed to serialize event"),
                Err(SendMessageErr::IoError) => {
                    warn!("Failed to send event. Connection closed? Closing it.");
                    *cancelled.borrow_mut() = true;
                }
            }
        });
    }
}

/// Wrapper for sending protobuf messages over a Unix stream.
/// Appends a length prefix before the serialized message.
struct Message<T: protobuf::Serialize> {
    payload: T,
}

enum SendMessageErr {
    Malformed,
    IoError,
}

impl<T: protobuf::Serialize> Message<T> {
    #[inline]
    fn new(payload: T) -> Self {
        Self { payload }
    }

    fn write_to_stream(&self, stream: &mut UnixStream) -> Result<(), SendMessageErr> {
        let bytes = self.payload.serialize().map_err(|_| SendMessageErr::Malformed)?;
        let msg_len: u64 = bytes.len().try_into().map_err(|_| SendMessageErr::Malformed)?;
        stream.write(&msg_len.to_le_bytes()).map_err(|_| SendMessageErr::IoError)?;
        stream.write_all(&bytes).map_err(|_| SendMessageErr::IoError)
    }
}

fn write_msg_to_stream<TPayload, TData>(data: TData, stream: &mut UnixStream) -> Result<(), SendMessageErr>
where
    TPayload: protobuf::Serialize,
    TData: Into<TPayload>,
{
    let payload = data.into();
    let msg = Message::new(payload);
    msg.write_to_stream(stream)
}

fn get_current_desktop_state() -> proto_rust::Clients {
    HyprCtl::default()
        .run(GetClientsCmd)
        .map(|clients| clients.into())
        .unwrap_or_else(|| {
            error!("Failed to get clients from Hyprland");
            proto_rust::Clients::default()
        })
}

fn window_events_socket_path() -> PathBuf {
    std::env::temp_dir().join("stil.sock")
}
