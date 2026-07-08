use std::path::Path;
use std::time::Instant;

use super::{
    progress::ScanProgress,
    types::{ScanContext, ScanResult},
    walker::walk_directory,
};

/// Scans a directory recursively.
///
/// This function coordinates the scan. It does not perform filesystem
/// traversal itself; that responsibility belongs to `walker.rs`.
pub fn scan_directory<F>(
    path: &Path,
    progress: F,
) -> Result<ScanResult, std::io::Error>
where
    F: FnMut(ScanProgress),
{
    let start = Instant::now();

    let context = ScanContext::new(path.to_path_buf());

    let mut result = ScanResult::default();

    let mut progress = progress;

    walk_directory(
        &context,
        &mut result,
        &mut progress,
    );

    result.elapsed_ms = start.elapsed().as_millis();

    Ok(result)
}