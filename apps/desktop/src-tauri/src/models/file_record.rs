use std::path::PathBuf;
use std::time::SystemTime;

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

    // ---------- Analysis ----------
    pub duplicate_group: Option<u64>,
}