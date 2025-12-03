use crate::{db::models::Application, service_locator::ServiceLocator};
use tracing::warn;

#[derive(Debug, Default)]
pub struct ApplicationRepo;

impl ApplicationRepo {
    pub fn get_by_id(&self, id: &str) -> Option<Application> {
        let sql = "SELECT * FROM applications WHERE id = ?1";
        ServiceLocator::db_conn()
            .query_one(sql, [id], |row| Application::try_from(row))
            .inspect_err(|err| warn!("Failed to query application by id {id}: {err}"))
            .ok()
    }

    pub fn get_pinned(&self) -> Vec<Application> {
        let sql = "SELECT * FROM applications WHERE pinned = 1";
        let conn = ServiceLocator::db_conn();
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

    pub fn add(&self, app: &Application) -> bool {
        let sql = "INSERT INTO applications (id, pinned) VALUES (?1, ?2)";
        ServiceLocator::db_conn()
            .execute(sql, rusqlite::params![app.id.as_str(), app.is_pinned])
            .inspect_err(|err| warn!("Failed to insert application {}: {err}", app.id))
            .is_ok()
    }

    pub fn update(&self, app: &Application) -> bool {
        let sql = "UPDATE applications SET pinned = ?1 WHERE id = ?2";
        ServiceLocator::db_conn()
            .execute(sql, rusqlite::params![app.is_pinned, app.id.as_str()])
            .inspect_err(|err| warn!("Failed to update application {}: {err}", app.id))
            .is_ok()
    }
}
