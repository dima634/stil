pub mod models;
mod schema;

use std::{
    ops::{Deref, DerefMut},
    sync::{Condvar, Mutex},
};

pub use schema::{migrate_down, migrate_up};

pub struct DbConnPool {
    pool_size: u32,
    pool: Mutex<Pool>,
    available: Condvar,
}

struct Pool {
    connections: Vec<rusqlite::Connection>,
    total_created: u32,
}

impl DbConnPool {
    pub fn new(pool_size: u32) -> Self {
        Self {
            pool_size,
            pool: Mutex::new(Pool {
                connections: Vec::new(),
                total_created: 0,
            }),
            available: Condvar::new(),
        }
    }

    /// Returns a connection from the pool. If no connections are available, it will block until one is returned.
    pub fn get_conn(&self) -> ConnectionGuard<'_> {
        // Try to get a connection from the pool
        // If not available block calling thread until one is returned
        let mut pool = self.pool.lock().expect("poisoned");

        if pool.total_created < self.pool_size {
            let conn = connect();
            pool.total_created += 1;

            return ConnectionGuard {
                pool: self,
                conn: Some(conn),
            };
        }

        while pool.connections.is_empty() {
            pool = self.available.wait(pool).expect("poisoned");
        }

        // Safe due to the is_empty check above
        let conn = pool.connections.pop().expect("no available connections");
        ConnectionGuard {
            pool: self,
            conn: Some(conn),
        }
    }

    fn return_conn(&self, conn: rusqlite::Connection) {
        let mut pool = self.pool.lock().expect("poisoned");
        pool.connections.push(conn);
        self.available.notify_one();
    }
}

pub struct ConnectionGuard<'a> {
    pool: &'a DbConnPool,
    conn: Option<rusqlite::Connection>,
}

impl<'a> Deref for ConnectionGuard<'a> {
    type Target = rusqlite::Connection;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.conn.as_ref().expect("should be Some until dropped")
    }
}

impl<'a> DerefMut for ConnectionGuard<'a> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.conn.as_mut().expect("should be Some until dropped")
    }
}

impl Drop for ConnectionGuard<'_> {
    #[inline]
    fn drop(&mut self) {
        self.pool
            .return_conn(self.conn.take().expect("should be Some until dropped"));
    }
}

pub fn connect() -> rusqlite::Connection {
    let home_dir = std::env::var("HOME").expect("HOME env var not set");
    let db_dir = std::path::PathBuf::from(home_dir).join(".local/share/stil");
    let db_path = db_dir.join("stil.sqlite");
    std::fs::create_dir_all(&db_dir).expect("failed to create database directory");
    rusqlite::Connection::open(db_path).expect("failed to open database connection")
}
