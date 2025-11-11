use super::EventParseErr;

#[derive(Debug, Clone)]
pub struct WorkspaceV2 {
    pub id: i32,
    pub name: String,
}

impl TryFrom<&str> for WorkspaceV2 {
    type Error = EventParseErr;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(EventParseErr::InvalidData);
        }

        let (id, name) = value
            .split_once(',')
            .ok_or(EventParseErr::InvalidData)?;

        Ok(WorkspaceV2 {
            id: id.parse().map_err(|_| EventParseErr::InvalidData)?,
            name: name.to_string(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct CreateWorkspaceV2(pub WorkspaceV2);

impl TryFrom<&str> for CreateWorkspaceV2 {
    type Error = EventParseErr;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(CreateWorkspaceV2(WorkspaceV2::try_from(value)?))
    }
}

#[derive(Debug, Clone)]
pub struct DestroyWorkspaceV2(pub WorkspaceV2);

impl TryFrom<&str> for DestroyWorkspaceV2 {
    type Error = EventParseErr;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(DestroyWorkspaceV2(WorkspaceV2::try_from(value)?))
    }
}
