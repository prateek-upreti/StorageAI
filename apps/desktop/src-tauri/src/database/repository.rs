//! Database repository layer.
//!
//! Responsibilities:
//! - Insert records
//! - Update records
//! - Delete records
//! - Query records
//! - Aggregate statistics

use rusqlite::Result;

use super::sqlite::open_database;

/// Verifies that the SQLite database can be opened.
pub fn initialize_repository() -> Result<()> {
    let _connection = open_database()?;

    Ok(())
}