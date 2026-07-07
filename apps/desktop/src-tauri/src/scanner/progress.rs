use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ScanProgress {
    pub files: u64,
    pub folders: u64,
    pub total_size: u64,
    pub current_path: String,
}