use std::path::Path;

use crate::{
    constants::APP_VERSION,
    scanner::scan_directory,
};

/// Returns the current application version.
#[tauri::command]
pub fn app_version() -> String {
    APP_VERSION.to_string()
}

/// Opens the native folder picker and returns the selected path.
#[tauri::command]
pub async fn select_folder(
    app: tauri::AppHandle,
) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;

    let folder = app
        .dialog()
        .file()
        .blocking_pick_folder();

    Ok(folder.map(|path| path.to_string()))
}

/// Scans a directory and returns scan statistics.
#[tauri::command]
pub fn start_scan(path: String) -> Result<(u64, u64, u64, u128, u64), String> {
    println!("Starting scan...");
    println!("Path: {}", path);

    let result = std::thread::spawn(move || {
        scan_directory(
            Path::new(&path),
            |_progress| {
                // We'll emit progress in the next step.
            },
        )
    })
    .join()
    .map_err(|_| "Scanner thread panicked".to_string())?
    .map_err(|e| e.to_string())?;

    println!();
    println!("========== Scan Finished ==========");
    println!("Files       : {}", result.files);
    println!("Folders     : {}", result.folders);
    println!("Skipped     : {}", result.skipped_folders);
    println!("Total Size  : {} bytes", result.total_size);
    println!("Elapsed Time: {} ms", result.elapsed_ms);
    println!("==================================");
    println!();

    Ok((
        result.files,
        result.folders,
        result.total_size,
        result.elapsed_ms,
        result.skipped_folders,
    ))
}