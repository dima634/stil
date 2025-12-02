use crate::{db::models, service_locator::ServiceLocator};
use rusqlite::params;
use tracing::trace;

pub fn migrate_up() -> Result<(), rusqlite::Error> {
    trace!("Migrating database up...");

    // TODO: validate that migrations order is correct
    let mut conn = ServiceLocator::db_conn();

    if migrations_table_exists(&mut conn)? {
        // Skip already applied migrations
        let last_migration = get_last_migration(&mut conn)?;
        let skip = last_migration.id as usize + 1;

        if skip >= migrations().len() {
            trace!("No new migrations to apply.");
            return Ok(());
        }

        apply_migrations(&mut conn, skip)?;
    } else {
        // No migrations have been applied yet, apply all migrations
        apply_migrations(&mut conn, 0)?;
    }

    trace!("Database migrated up successfully.");

    Ok(())
}

pub fn migrate_down() -> Result<(), rusqlite::Error> {
    trace!("Migrating database down...");

    let mut conn = ServiceLocator::db_conn();

    if migrations_table_exists(&mut conn)? {
        let last_migration = get_last_migration(&mut conn)?;
        let migrate_down = migrations()[last_migration.id as usize].down_sql;

        trace!("Reverting migration: {}", last_migration.name);

        if last_migration.id > 0 {
            let tx = conn.transaction()?;
            {
                tx.execute(migrate_down, [])?;
                tx.execute("DELETE FROM migrations WHERE id = ?1", params![last_migration.id])?;
            }
            tx.commit()?;
        } else {
            conn.execute(migrate_down, [])?;
        }

        trace!("Database migrated down successfully.");
    } else {
        trace!("No migrations to roll back.");
    }

    Ok(())
}

fn migrations_table_exists(conn: &mut rusqlite::Connection) -> Result<bool, rusqlite::Error> {
    let select_migrations_table = "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='migrations'";
    let count: usize = conn.query_one(select_migrations_table, [], |row| row.get(0))?;
    Ok(count > 0)
}

fn get_last_migration(conn: &mut rusqlite::Connection) -> Result<models::Migration, rusqlite::Error> {
    let select_last_migration = "SELECT * FROM migrations ORDER BY id DESC LIMIT 1";
    conn.query_row(select_last_migration, [], |row| models::Migration::try_from(row))
}

fn apply_migrations(conn: &mut rusqlite::Connection, skip: usize) -> Result<(), rusqlite::Error> {
    let tx = conn.transaction()?;
    {
        for migration in migrations().iter().skip(skip) {
            trace!("Applying migration: {}", migration.name);
            tx.execute(migration.up_sql, [])?;
        }

        let mut insert_migration = tx.prepare("INSERT INTO migrations(id, name) VALUES (?1, ?2)")?;

        for (index, migration) in migrations().iter().enumerate().skip(skip) {
            trace!("Recording migration: {}", migration.name);
            insert_migration.execute(params![index, migration.name])?;
        }
    }
    tx.commit()
}

struct Migration {
    name: &'static str,
    up_sql: &'static str,
    down_sql: &'static str,
}

const fn migrations() -> &'static [Migration] {
    &[
        Migration {
            name: "create_migrations_table",
            up_sql: "
                    CREATE TABLE migrations(
                        id INTEGER PRIMARY KEY,
                        name TEXT NOT NULL,
                        applied_at TEXT DEFAULT (DATETIME('now'))
                    );
                ",
            down_sql: "
                    DROP TABLE IF EXISTS migrations;
                ",
        },
        Migration {
            name: "create_application_table",
            up_sql: "
                CREATE TABLE applications (
                    id TEXT PRIMARY KEY,
                    pinned INTEGER
                );
            ",
            down_sql: "
                DROP TABLE applications;
            ",
        },
        // WARN: Keep migrations in order.
        // You should not remove or edit existing migrations.
        // Instead, add new migrations to the end of the list.
    ]
}
