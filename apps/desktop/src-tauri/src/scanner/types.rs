use std::path::PathBuf;

use crate::models::FileRecord;

/// Configuration used by the scanner.
#[derive(Debug, Clone)]
pub struct ScanContext {
    /// Root directory to scan.
    pub root: PathBuf,

    /// Number of FileRecords collected before flushing to the caller.
    pub batch_size: usize,

    /// Progress callback interval.
    pub progress_interval: u64,
}

impl ScanContext {
    pub fn new(root: PathBuf) -> Self {
        Self {
            root,
            batch_size: 500,
            progress_interval: 500,
        }
    }
}

/// Result returned after a scan finishes.
#[derive(Debug)]
pub struct ScanResult {
    pub files: u64,
    pub folders: u64,
    pub skipped_folders: u64,
    pub total_size: u64,
    pub elapsed_ms: u128,

    /// All discovered files.
    pub records: Vec<FileRecord>,
}

impl Default for ScanResult {
    fn default() -> Self {
        Self {
            files: 0,
            folders: 0,
            skipped_folders: 0,
            total_size: 0,
            elapsed_ms: 0,
            records: Vec::new(),
        }
    }
}