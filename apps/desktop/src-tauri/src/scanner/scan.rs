use std::fs;
use std::path::Path;

use super::progress::ScanProgress;

/// Final scan result.
#[derive(Debug, Clone)]
pub struct ScanResult {
    pub files: u64,
    pub folders: u64,
    pub total_size: u64,
    pub elapsed_ms: u128,
    pub skipped_folders: u64,
}

/// Scan a directory recursively.
pub fn scan_directory<F>(
    path: &Path,
    mut progress: F,
) ->Result<ScanResult,std::io::Error>
where
    F: FnMut(ScanProgress),
{
    let start = std::time::Instant::now();

    let mut result = ScanResult {
        files: 0,
        folders: 0,
        total_size: 0,
        elapsed_ms: 0,
        skipped_folders: 0,
    };

    scan_recursive(path, &mut result, &mut progress);

    result.elapsed_ms = start.elapsed().as_millis();

    Ok(result)
}

fn scan_recursive<F>(
    path: &Path,
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
            Ok(e) => e,
            Err(_) => continue,
        };

        let entry_path = entry.path();

        if entry_path.is_dir() {
            result.folders += 1;

            scan_recursive(&entry_path, result, progress);
        } else if entry_path.is_file() {
            result.files += 1;

            if let Ok(metadata) = entry.metadata() {
                result.total_size += metadata.len();
            }

            if result.files % 500 == 0 {
                progress(ScanProgress {
                    files: result.files,
                    folders: result.folders,
                    total_size: result.total_size,
                    current_path: entry_path.display().to_string(),
                });
            }
        }
    }
}