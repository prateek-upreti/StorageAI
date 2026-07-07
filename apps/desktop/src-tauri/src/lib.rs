mod commands;
mod constants;
mod scanner;

use commands::{
    app_version,
    select_folder,
    start_scan,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            app_version,
            select_folder,
            start_scan,
        ])
        .setup(|app| {
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