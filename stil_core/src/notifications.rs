use crate::{SystemEvent, system_events::Notification};
use std::{collections::HashMap, sync::mpsc::Sender};
use tracing::error;
use zbus::{blocking::connection, interface, object_server::SignalEmitter};

#[derive(Debug)]
pub struct NotificationsService {
    conn: zbus::blocking::Connection,
}

impl NotificationsService {
    pub fn new(sender: Sender<SystemEvent>) -> Option<Self> {
        Self::start_zbus_server(sender)
            .inspect_err(|e| error!("Notifications Service: failed to start zbus server. Inner error: {}", e))
            .map(|conn| Self { conn })
            .ok()
    }

    fn start_zbus_server(sender: Sender<SystemEvent>) -> zbus::Result<connection::Connection> {
        connection::Builder::session()?
            .name("org.freedesktop.Notifications")?
            .serve_at("/org/freedesktop/Notifications", NotificationsInterface(sender))?
            .build()
    }
}

struct NotificationsInterface(Sender<SystemEvent>);

#[interface(name = "org.freedesktop.Notifications")]
impl NotificationsInterface {
    fn get_capabilities(&self) -> Vec<&str> {
        vec!["body"]
    }

    fn notify(
        &self,
        app_name: &str,
        replaces_id: u32,
        app_icon: &str,
        summary: &str,
        body: &str,
        actions: Vec<&str>,
        hints: HashMap<&str, zbus::zvariant::Value>,
        expire_timeout: i32,
    ) -> u32 {
        let event = SystemEvent::Notification(Notification {
            summary: summary.to_string(),
            body: if body.is_empty() { None } else { Some(body.to_string()) },
            icon: if app_icon.is_empty() {
                None
            } else {
                Some(app_icon.to_string())
            },
        });

        if let Err(e) = self.0.send(event) {
            error!(
                "Notifications Service: failed to send notification event. Inner error: {}",
                e
            );
        }

        1 // Return a dummy notification ID TODO: implement proper ID management
    }

    fn close_notification(&self, id: u32) {
        todo!()
    }

    fn get_server_information(&self) -> (&str, &str, &str, &str) {
        ("Stil Notifications", "Stil", "1.0", "1.2")
    }

    #[zbus(signal)]
    async fn notification_closed(emitter: &SignalEmitter<'_>, id: u32, reason: CloseReason) -> zbus::Result<()>;

    #[zbus(signal)]
    async fn action_invoked(emitter: &SignalEmitter<'_>, id: u32, action: &str) -> zbus::Result<()>;

    #[zbus(signal)]
    async fn activation_token(emitter: &SignalEmitter<'_>, id: u32, activation_token: &str) -> zbus::Result<()>;
}

#[derive(Debug, serde::Serialize, zbus::zvariant::Type)]
pub enum CloseReason {
    Expired = 1,
    Dismissed = 2,
    ClosedByCall = 3,
    Undefined = 4,
}
