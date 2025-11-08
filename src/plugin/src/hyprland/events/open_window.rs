use super::EventParseErr;

#[derive(Debug, Clone)]
pub struct OpenWindow {
    pub window_address: String,
    pub workspace_name: String,
    pub window_class: String,
    pub window_title: String,
}

impl TryFrom<&str> for OpenWindow {
    type Error = EventParseErr;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts = value.split(',');
        let window_address = parts.next().ok_or(EventParseErr::InvalidData)?.to_string();
        let workspace_name = parts.next().ok_or(EventParseErr::InvalidData)?.to_string();
        let window_class = parts.next().ok_or(EventParseErr::InvalidData)?.to_string();
        let window_title = parts.next().ok_or(EventParseErr::InvalidData)?.to_string();

        Ok(OpenWindow {
            window_address,
            workspace_name,
            window_class,
            window_title,
        })
    }
}
