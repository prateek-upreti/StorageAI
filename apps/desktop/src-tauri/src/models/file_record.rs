use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

/// Shared domain model representing a discovered file.
#[derive(Debug, Clone)]
pub struct FileRecord {
    // ---------- Identity ----------

    pub path: PathBuf,
    pub file_name: String,
    pub extension: String,

    // ---------- Filesystem ----------

    pub size: u64,

    pub created: Option<SystemTime>,
    pub modified: Option<SystemTime>,
    pub accessed: Option<SystemTime>,

    pub is_hidden: bool,
    pub is_read_only: bool,
    pub is_symlink: bool,

    // ---------- Hashing ----------

    pub quick_hash: Option<String>,
    pub full_hash: Option<String>,

    // ---------- Metadata ----------

    pub mime_type: Option<String>,

    // ---------- Duplicate Detection ----------

    pub duplicate_group: Option<i64>,
}

impl FileRecord {
    /// Absolute path as String.
    pub fn path_string(&self) -> String {
        self.path.to_string_lossy().to_string()
    }

    /// Created time as unix timestamp.
    pub fn created_timestamp(&self) -> Option<i64> {
        system_time_to_timestamp(self.created)
    }

    /// Modified time as unix timestamp.
    pub fn modified_timestamp(&self) -> Option<i64> {
        system_time_to_timestamp(self.modified)
    }

    /// Accessed time as unix timestamp.
    pub fn accessed_timestamp(&self) -> Option<i64> {
        system_time_to_timestamp(self.accessed)
    }
}

fn system_time_to_timestamp(
    time: Option<SystemTime>,
) -> Option<i64> {
    time.and_then(|t| {
        t.duration_since(UNIX_EPOCH)
            .ok()
            .map(|d| d.as_secs() as i64)
    })
}