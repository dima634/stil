use crate::db::models::Application;
use std::rc::Rc;

#[derive(Debug)]
pub struct ApplicationRepo {
    conn: Rc<rusqlite::Connection>,
}

impl Default for ApplicationRepo {
    #[inline]
    fn default() -> Self {
        Self {
            conn: crate::db::connection(),
        }
    }
}

impl ApplicationRepo {
    pub fn get_by_id(&self, id: &str) -> Option<Application> {
        let sql = "SELECT * FROM applications WHERE id = ?1";
        self.conn.query_one(sql, [id], |row| Application::try_from(row)).ok()
    }

    pub fn add(&self, app: &Application) -> bool {
        let sql = "INSERT INTO applications (id, is_pinned) VALUES (?1, ?2)";
        self.conn
            .execute(sql, rusqlite::params![app.id.as_str(), app.is_pinned])
            .is_ok()
    }

    pub fn update(&self, app: &Application) -> bool {
        let sql = "UPDATE applications SET is_pinned = ?1 WHERE id = ?2";
        self.conn
            .execute(sql, rusqlite::params![app.is_pinned, app.id.as_str()])
            .is_ok()
    }
}
