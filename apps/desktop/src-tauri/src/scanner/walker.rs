use std::fs;
use std::path::Path;

use crate::{
    models::FileRecord,
    scanner::{
        progress::ScanProgress,
        types::{ScanContext, ScanResult},
    },
};

/// Starts walking the directory tree.
pub fn walk_directory<F>(
    context: &ScanContext,
    result: &mut ScanResult,
    progress: &mut F,
)
where
    F: FnMut(ScanProgress),
{
    walk_recursive(context.root.as_path(), context, result, progress);
}

/// Recursively walks a directory.
fn walk_recursive<F>(
    path: &Path,
    context: &ScanContext,
    result: &mut ScanResult,
    progress: &mut F,
)
where
    F: FnMut(ScanProgress),
{
    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(_) => {
            result.skipped_folders += 1;
            return;
        }
    };

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };

        let entry_path = entry.path();

        if entry_path.is_dir() {
            result.folders += 1;
            walk_recursive(&entry_path, context, result, progress);
            continue;
        }

        if !entry_path.is_file() {
            continue;
        }

        let metadata = match entry.metadata() {
            Ok(metadata) => metadata,
            Err(_) => continue,
        };

        result.files += 1;
        result.total_size += metadata.len();

        let record = FileRecord {
            path: entry_path.clone(),
            file_name: entry_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            extension: entry_path
                .extension()
                .map(|ext| ext.to_string_lossy().to_string())
                .unwrap_or_default(),
            size: metadata.len(),
            created: metadata.created().ok(),
            modified: metadata.modified().ok(),
            accessed: metadata.accessed().ok(),
            is_hidden: false, // Windows support will be added later
            is_read_only: metadata.permissions().readonly(),
            is_symlink: entry
                .file_type()
                .map(|t| t.is_symlink())
                .unwrap_or(false),
            quick_hash: None,
            full_hash: None,
            mime_type: None,
            duplicate_group: None,
        };

        result.records.push(record);

        if result.files % context.progress_interval == 0 {
            progress(ScanProgress {
                files: result.files,
                folders: result.folders,
                total_size: result.total_size,
                current_path: entry_path.display().to_string(),
            });
        }
    }
}