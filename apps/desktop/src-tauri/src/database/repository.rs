//! Database repository layer.

use rusqlite::{params, Connection, Result};

use crate::models::FileRecord;

use super::sqlite::open_database;

/// Repository for all SQLite database operations.
pub struct Repository {
    connection: Connection,
}

impl Repository {
    /// Creates a new repository instance.
    pub fn new() -> Result<Self> {
        Ok(Self {
            connection: open_database()?,
        })
    }

    /// Returns the SQLite connection.
    pub fn connection(&self) -> &Connection {
        &self.connection
    }

    /// Starts a new scan session.
    pub fn insert_scan_session(
        &mut self,
        scanned_path: &str,
        started_at: &str,
    ) -> Result<i64> {
        self.connection.execute(
            "
            INSERT INTO scan_sessions (
                started_at,
                scanned_path
            )
            VALUES (?1, ?2)
            ",
            params![
                started_at,
                scanned_path,
            ],
        )?;

        Ok(self.connection.last_insert_rowid())
    }

    /// Finishes a scan session.
    pub fn finish_scan_session(
        &mut self,
        session_id: i64,
        finished_at: &str,
        files: u64,
        folders: u64,
        total_size: u64,
        skipped: u64,
        duration_ms: u128,
    ) -> Result<()> {
        self.connection.execute(
            "
            UPDATE scan_sessions
            SET
                finished_at = ?1,
                files = ?2,
                folders = ?3,
                total_size = ?4,
                skipped = ?5,
                duration_ms = ?6
            WHERE id = ?7
            ",
            params![
                finished_at,
                files as i64,
                folders as i64,
                total_size as i64,
                skipped as i64,
                duration_ms as i64,
                session_id,
            ],
        )?;

        Ok(())
    }

    /// Bulk inserts file records using a single transaction.
    pub fn bulk_insert_file_records(
        &mut self,
        scan_session_id: i64,
        records: &[FileRecord],
    ) -> Result<()> {
        println!("Bulk inserting {} records...", records.len());

        let tx = self.connection.transaction()?;

        {
            let mut stmt = tx.prepare(
                "
                INSERT INTO file_records (
                    scan_session_id,
                    path,
                    file_name,
                    extension,
                    size,
                    created_at,
                    modified_at,
                    accessed_at,
                    is_hidden,
                    is_read_only,
                    is_symlink
                )
                VALUES (
                    ?1, ?2, ?3, ?4, ?5,
                    ?6, ?7, ?8, ?9, ?10, ?11
                )
                ",
            )?;

            for (index, record) in records.iter().enumerate() {
                if let Err(e) = stmt.execute(params![
                    scan_session_id,
                    record.path_string(),
                    &record.file_name,
                    &record.extension,
                    record.size as i64,
                    record.created_timestamp(),
                    record.modified_timestamp(),
                    record.accessed_timestamp(),
                    record.is_hidden,
                    record.is_read_only,
                    record.is_symlink,
                ]) {
                    println!(
                        "Failed to insert record #{}",
                        index
                    );
                    println!("Path: {}", record.path_string());
                    println!("SQLite error: {}", e);

                    return Err(e);
                }
            }
        }

        tx.commit()?;

        println!("Bulk insert committed.");

        Ok(())
    }
}

/// Initializes the repository.
pub fn initialize_repository() -> Result<()> {
    Repository::new()?;
    Ok(())
}