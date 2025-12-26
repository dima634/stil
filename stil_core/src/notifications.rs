use crate::{
    SystemEvent,
    system_events::{Image, Notification},
};
use std::{
    collections::HashMap,
    sync::{atomic::AtomicU32, mpsc::Sender},
};
use tracing::{error, info};
use zbus::zvariant::Value;
use zbus::{blocking::connection, interface, object_server::SignalEmitter};

#[derive(Debug)]
pub struct NotificationsService {
    _conn: zbus::blocking::Connection, // keep zbus server alive
}

impl NotificationsService {
    pub fn new(sender: Sender<SystemEvent>) -> Option<Self> {
        Self::start_zbus_server(sender)
            .inspect_err(|e| error!("Notifications Service: failed to start zbus server. Inner error: {}", e))
            .map(|_conn| Self { _conn })
            .ok()
    }

    fn start_zbus_server(sender: Sender<SystemEvent>) -> zbus::Result<connection::Connection> {
        connection::Builder::session()?
            .name("org.freedesktop.Notifications")?
            .serve_at(
                "/org/freedesktop/Notifications",
                NotificationsInterface {
                    sender,
                    next_notification_id: AtomicU32::new(1),
                },
            )?
            .build()
    }
}

struct NotificationsInterface {
    sender: Sender<SystemEvent>,
    next_notification_id: AtomicU32,
}

#[interface(name = "org.freedesktop.Notifications")]
impl NotificationsInterface {
    fn get_capabilities(&self) -> Vec<&str> {
        vec!["body", "icon-static"]
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
            image: parse_image_data(&hints),
        });

        if let Err(e) = self.sender.send(event) {
            error!("Failed to send notification event. Inner error: {}", e);
        }

        self.next_notification_id
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    }

    fn close_notification(&self, id: u32) {
        info!("Not implemented");
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

/// Parse image-data from notification hints
/// Tries "image-data", "image_data", and "icon_data" keys per the spec
fn parse_image_data(hints: &HashMap<&str, Value>) -> Option<Image> {
    // Try different keys per freedesktop spec
    let value = hints
        .get("image-data")
        .or_else(|| hints.get("image_data"))
        .or_else(|| hints.get("icon_data"))?;

    // image-data is a struct: (iiibiiay)
    let structure = value.downcast_ref::<zbus::zvariant::Structure>().ok()?;
    let fields = structure.fields();

    if fields.len() != 7 {
        return None;
    }

    let width = fields[0].downcast_ref::<i32>().ok()?;
    let height = fields[1].downcast_ref::<i32>().ok()?;
    let row_stride = fields[2].downcast_ref::<i32>().ok()?;
    let has_alpha = fields[3].downcast_ref::<bool>().ok()?;
    let bits_per_sample = fields[4].downcast_ref::<i32>().ok()?;
    let channels = fields[5].downcast_ref::<i32>().ok()?;
    let data = fields[6].downcast_ref::<zbus::zvariant::Array>().ok()?;

    let data: Option<Vec<u8>> = data.iter().map(|v| v.downcast_ref::<u8>().ok()).collect();
    let data = data?;

    Some(Image {
        width,
        height,
        row_stride,
        has_alpha,
        bits_per_sample,
        channels,
        data,
    })
}
