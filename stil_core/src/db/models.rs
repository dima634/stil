use chrono::{DateTime, NaiveDateTime, Utc};

#[derive(Debug)]
pub struct Application {
    pub id: String,
    pub is_pinned: bool,
}

impl TryFrom<&rusqlite::Row<'_>> for Application {
    type Error = rusqlite::Error;

    fn try_from(value: &rusqlite::Row<'_>) -> Result<Self, Self::Error> {
        Ok(Application {
            id: value.get("id")?,
            is_pinned: value.get("is_pinned")?,
        })
    }
}

#[derive(Debug)]
pub struct Migration {
    pub id: i64,
    pub name: String,
    pub applied_at: DateTime<Utc>,
}

impl TryFrom<&rusqlite::Row<'_>> for Migration {
    type Error = rusqlite::Error;

    fn try_from(value: &rusqlite::Row<'_>) -> Result<Self, Self::Error> {
        let applied_at_str: String = value.get("applied_at")?;
        let applied_at = NaiveDateTime::parse_from_str(&applied_at_str, "%Y-%m-%d %H:%M:%S")
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e)))?
            .and_utc();

        Ok(Migration {
            id: value.get("id")?,
            name: value.get("name")?,
            applied_at,
        })
    }
}
