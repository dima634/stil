use super::EventParseErr;

#[derive(Debug, Clone)]
pub struct WorkspaceV2 {
    id: i32,
    name: String,
}

impl WorkspaceV2 {
    #[inline]
    pub fn id(&self) -> i32 {
        self.id
    }

    #[inline]
    pub fn name(&self) -> &String {
        &self.name
    }
}

impl TryFrom<&str> for WorkspaceV2 {
    type Error = EventParseErr;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(EventParseErr::InvalidData);
        }

        let (id, name) = value.split_once(',').ok_or(EventParseErr::InvalidData)?;

        Ok(WorkspaceV2 {
            id: id.parse().map_err(|_| EventParseErr::InvalidData)?,
            name: name.to_string(),
        })
    }
}
