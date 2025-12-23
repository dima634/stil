use std::sync::LazyLock;
use tracing::error;
use zbus::blocking::*;

pub fn system_dbus() -> Option<&'static Connection> {
    static INSTANCE: LazyLock<Option<Connection>> = LazyLock::new(|| match Connection::system() {
        Ok(conn) => Some(conn),
        Err(e) => {
            error!("Failed to connect to system D-Bus. Inner error: {}", e);
            None
        }
    });

    INSTANCE.as_ref()
}
