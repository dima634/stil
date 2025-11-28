use std::{path::PathBuf, rc::Rc};

mod models;
mod schema;

pub use models::*;
pub use schema::{migrate_down, migrate_up};

/// Returns a shared thread-local database connection
#[inline]
pub fn connection() -> Rc<rusqlite::Connection> {
    CONN.with(|conn| conn.clone())
}

/// Returns a new owned database connection
#[inline]
pub fn connection_owned() -> rusqlite::Connection {
    connect()
}

thread_local! {
    static CONN: Rc<rusqlite::Connection> = Rc::new(connect());
}

fn connect() -> rusqlite::Connection {
    let home_dir = std::env::var("HOME").expect("HOME env var not set");
    let db_dir = PathBuf::from(home_dir).join(".local/share/stil");
    let db_path = db_dir.join("stil.sqlite");
    std::fs::create_dir_all(&db_dir).expect("failed to create database directory");
    rusqlite::Connection::open(db_path).expect("failed to open database connection")
}
