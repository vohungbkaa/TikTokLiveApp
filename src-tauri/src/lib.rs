use tauri::Manager;

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
        .setup(|app| {
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                match db::init_db(&handle).await {
                    Ok(pool) => {
                        handle.manage(db::DbState { pool });
                        tracing::info!("Database initialized and state managed.");
                    }
                    Err(e) => {
                        tracing::error!("Failed to initialize database: {}", e);
                    }
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::app::app_health_check,
            db::backup_db,
            commands::session::create_session,
            commands::session::get_sessions,
            commands::session::start_session,
            commands::session::end_session,
            commands::session::open_session_history,
            commands::session::set_session_products
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
