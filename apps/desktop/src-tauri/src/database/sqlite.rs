//! SQLite database connection layer.
//!
//! Responsibilities:
//! - Open the StorageAI database
//! - Create the database if it does not exist
//! - Initialize the database schema

use rusqlite::{Connection, Result};

/// Opens (or creates) the StorageAI SQLite database.
pub fn open_database() -> Result<Connection> {
    let connection = Connection::open("storageai.db")?;

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
        ",
    )?;

    connection.execute_batch(
        "
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
        ",
    )?;

    connection.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS file_records (
            id                  INTEGER PRIMARY KEY AUTOINCREMENT,

            scan_session_id     INTEGER NOT NULL,

            path                TEXT NOT NULL UNIQUE,
            file_name           TEXT NOT NULL,
            extension           TEXT,

            size                INTEGER NOT NULL,

            created_at          TEXT,
            modified_at         TEXT,
            accessed_at         TEXT,

            is_hidden           INTEGER NOT NULL DEFAULT 0,
            is_read_only        INTEGER NOT NULL DEFAULT 0,
            is_symlink          INTEGER NOT NULL DEFAULT 0,

            FOREIGN KEY(scan_session_id)
                REFERENCES scan_sessions(id)
                ON DELETE CASCADE
        );
        ",
    )?;

    Ok(())
}