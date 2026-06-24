pub mod commands;
pub mod connectors;
pub mod db;
pub mod diagnostics;
pub mod events;
pub mod orders;
pub mod parser;
pub mod products;
pub mod sessions;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    diagnostics::logging::init_logger();
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::app::app_health_check
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
