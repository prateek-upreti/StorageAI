//! SQLite database connection layer.

use std::fs;

use dirs::data_local_dir;
use rusqlite::{Connection, Result};

/// Opens (or creates) the StorageAI SQLite database.
pub fn open_database() -> Result<Connection> {
    let mut database_path = data_local_dir()
        .expect("Unable to locate Local AppData");

    database_path.push("StorageAI");

    fs::create_dir_all(&database_path)
        .expect("Unable to create StorageAI data directory");

    database_path.push("storageai.db");

    println!(
        "StorageAI Database: {}",
        database_path.display()
    );

    let connection = Connection::open(database_path)?;

    initialize_database(&connection)?;

    Ok(connection)
}

/// Creates the database schema if it does not already exist.
fn initialize_database(connection: &Connection) -> Result<()> {
    connection.execute_batch(
        "
        PRAGMA foreign_keys = ON;
        PRAGMA journal_mode = WAL;
        PRAGMA synchronous = NORMAL;

        CREATE TABLE IF NOT EXISTS scan_sessions (
            id              INTEGER PRIMARY KEY AUTOINCREMENT,
            started_at      TEXT NOT NULL,
            finished_at     TEXT,
            scanned_path    TEXT NOT NULL,
            files           INTEGER NOT NULL DEFAULT 0,
            folders         INTEGER NOT NULL DEFAULT 0,
            total_size      INTEGER NOT NULL DEFAULT 0,
            skipped         INTEGER NOT NULL DEFAULT 0,
            duration_ms     INTEGER NOT NULL DEFAULT 0
        );

        CREATE TABLE IF NOT EXISTS file_records (
            id                  INTEGER PRIMARY KEY AUTOINCREMENT,

            scan_session_id     INTEGER NOT NULL,

            path                TEXT NOT NULL,
            file_name           TEXT NOT NULL,
            extension           TEXT,

            size                INTEGER NOT NULL,

            created_at          INTEGER,
            modified_at         INTEGER,
            accessed_at         INTEGER,

            is_hidden           INTEGER NOT NULL DEFAULT 0,
            is_read_only        INTEGER NOT NULL DEFAULT 0,
            is_symlink          INTEGER NOT NULL DEFAULT 0,

            FOREIGN KEY(scan_session_id)
                REFERENCES scan_sessions(id)
                ON DELETE CASCADE
        );

        CREATE INDEX IF NOT EXISTS idx_file_records_path
            ON file_records(path);

        CREATE INDEX IF NOT EXISTS idx_file_records_scan
            ON file_records(scan_session_id);

        CREATE INDEX IF NOT EXISTS idx_file_records_size
            ON file_records(size);
        ",
    )?;

    Ok(())
}