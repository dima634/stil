use super::EventParseErr;

#[derive(Debug, Clone)]
pub struct ActiveWindow {
    pub address: usize,
}

impl TryFrom<&str> for ActiveWindow {
    type Error = EventParseErr;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let address = usize::from_str_radix(value, 16).map_err(|_| EventParseErr::InvalidData)?;
        
        Ok(ActiveWindow {
            address,
        })
    }
}
