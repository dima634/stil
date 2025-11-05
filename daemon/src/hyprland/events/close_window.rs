use super::EventParseErr;

#[derive(Debug, Clone)]
pub struct CloseWindow {
    pub window_address: String,
}

impl TryFrom<&str> for CloseWindow {
    type Error = EventParseErr;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(EventParseErr::InvalidData);
        }

        Ok(CloseWindow {
            window_address: value.to_string(),
        })
    }
}

impl Into<proto_rust::CloseWindowEvent> for CloseWindow {
    fn into(self) -> proto_rust::CloseWindowEvent {
        let mut close_window_event = proto_rust::CloseWindowEvent::default();
        close_window_event.set_address(self.window_address);
        close_window_event
    }
}
