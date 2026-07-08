mod commands;
mod constants;
mod database;
mod scanner;
mod models;

use commands::{
    app_version,
    select_folder,
    start_scan,
};
use database::repository::initialize_repository;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            app_version,
            select_folder,
            start_scan,
        ])
        .setup(|app| {
            initialize_repository()
                .expect("Failed to initialize StorageAI database");

            app.handle().plugin(tauri_plugin_dialog::init())?;

            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}