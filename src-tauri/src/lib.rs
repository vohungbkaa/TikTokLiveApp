use tauri::Manager;

pub mod commands;
pub mod connectors;
pub mod customers;
pub mod db;
pub mod diagnostics;
pub mod events;
pub mod inventory;
pub mod orders;
pub mod parser;
pub mod products;
pub mod selling_modes;
pub mod sessions;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    diagnostics::logging::init_logger();
    
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let handle = app.handle().clone();
            
            tauri::async_runtime::spawn(async move {
                match db::init_db(&handle).await {
                    Ok(pool) => {
                        let supervisor = connectors::supervisor::ConnectorSupervisor::new(handle.clone(), pool.clone());
                        handle.manage(supervisor);
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
            commands::session::delete_session,
            commands::session::open_session_history,
            commands::session::set_session_products,
            commands::product::list_products,
            commands::product::create_product,
            commands::product::update_stock,
            commands::event::get_session_events,
            commands::event::test_ingest_event,
            commands::parser::debug_parse_comment,
            commands::order::list_orders_by_session,
            commands::order::get_order_summary,
            commands::customer::search_customers,
            commands::customer::update_customer_profile
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
