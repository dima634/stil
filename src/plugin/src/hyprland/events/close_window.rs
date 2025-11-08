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
