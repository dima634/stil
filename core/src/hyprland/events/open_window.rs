use super::EventParseErr;

#[derive(Debug, Clone)]
pub struct OpenWindow {
    pub window_address: usize,
    pub workspace_name: String,
    pub window_class: String,
    pub window_title: String,
}

impl OpenWindow {
    #[inline]
    pub fn address(&self) -> usize {
        self.window_address
    }

    #[inline]
    pub fn workspace_name(&self) -> &str {
        &self.workspace_name
    }

    #[inline]
    pub fn class(&self) -> &str {
        &self.window_class
    }

    #[inline]
    pub fn title(&self) -> &str {
        &self.window_title
    }
}

impl TryFrom<&str> for OpenWindow {
    type Error = EventParseErr;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts = value.split(',');
        let window_address = parts
            .next()
            .map(|v| v.parse().ok())
            .flatten()
            .ok_or_else(|| EventParseErr::InvalidData)?;
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
