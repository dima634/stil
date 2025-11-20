use super::EventParseErr;

#[derive(Debug, Clone)]
pub struct MoveWindowV2 {
    pub window_address: usize,
    pub workspace_id: i32,
    pub workspace_name: String,
}

impl TryFrom<&str> for MoveWindowV2 {
    type Error = EventParseErr;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts = value.split(',');
        let addr_str = parts.next().ok_or(EventParseErr::InvalidData)?;
        let window_address = usize::from_str_radix(addr_str, 16).map_err(|_| EventParseErr::InvalidData)?;
        let workspace_id = parts
            .next()
            .ok_or(EventParseErr::InvalidData)?
            .parse()
            .map_err(|_| EventParseErr::InvalidData)?;
        let workspace_name = parts.next().ok_or(EventParseErr::InvalidData)?.to_string();

        Ok(Self {
            window_address,
            workspace_id,
            workspace_name,
        })
    }
}
