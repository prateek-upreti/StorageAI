use std::path::Path;

use chrono::Utc;

use crate::{
    constants::APP_VERSION,
    database::repository::Repository,
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

/// Scans a directory, stores the results in SQLite,
/// and returns scan statistics.
#[tauri::command]
pub fn start_scan(
    path: String,
) -> Result<(u64, u64, u64, u128, u64), String> {
    println!("Starting scan...");
    println!("Path: {}", path);

    let started_at = Utc::now().to_rfc3339();

    let mut repository =
        Repository::new().map_err(|e| e.to_string())?;

    let session_id = repository
        .insert_scan_session(&path, &started_at)
        .map_err(|e| e.to_string())?;

    let result = scan_directory(
        Path::new(&path),
        |_progress| {
            // Progress events will be emitted later.
        },
    )
    .map_err(|e| e.to_string())?;

    repository
        .bulk_insert_file_records(
            session_id,
            &result.records,
        )
        .map_err(|e| e.to_string())?;

    let finished_at = Utc::now().to_rfc3339();

    repository
        .finish_scan_session(
            session_id,
            &finished_at,
            result.files,
            result.folders,
            result.total_size,
            result.skipped_folders,
            result.elapsed_ms,
        )
        .map_err(|e| e.to_string())?;

    println!();
    println!("========== Scan Finished ==========");
    println!("Session ID  : {}", session_id);
    println!("Files       : {}", result.files);
    println!("Folders     : {}", result.folders);
    println!("Skipped     : {}", result.skipped_folders);
    println!("Stored      : {}", result.records.len());
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