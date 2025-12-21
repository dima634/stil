use crate::db::{self, models::Application};
use tracing::warn;

#[derive(Debug, Default)]
pub struct ApplicationRepo;

impl ApplicationRepo {
    pub fn get_pinned(&self) -> Vec<Application> {
        let sql = "SELECT * FROM applications WHERE pinned = 1";
        let conn = db::pool().get_conn();
        let mut stmt = match conn.prepare(sql) {
            Ok(stmt) => stmt,
            Err(err) => {
                warn!("Failed to query pinned applications: {err}");
                return Vec::new();
            }
        };
        stmt.query_and_then([], |row| Application::try_from(row))
            .map(|apps| apps.filter_map(Result::ok).collect())
            .inspect_err(|err| warn!("Failed to query pinned applications: {err}"))
            .unwrap_or_default()
    }
}
