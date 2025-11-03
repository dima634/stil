use super::EventParseErr;

#[derive(Debug, Clone)]
pub struct ActiveWindow {
    pub window_class: String,
    pub window_title: String,
}

impl TryFrom<&str> for ActiveWindow {
    type Error = EventParseErr;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts = value.split(',');
        let window_class = parts.next().ok_or(EventParseErr::InvalidData)?.to_string();
        let window_title = parts.next().ok_or(EventParseErr::InvalidData)?.to_string();

        Ok(ActiveWindow {
            window_class,
            window_title,
        })
    }
}
