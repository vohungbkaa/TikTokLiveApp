use sqlx::{sqlite::{SqliteConnectOptions, SqlitePoolOptions, SqliteJournalMode}, SqlitePool};
use std::str::FromStr;
use tauri::{AppHandle, Manager};

pub mod repositories;

pub struct DbState {
    pub pool: SqlitePool,
}

pub async fn init_db(app_handle: &AppHandle) -> Result<SqlitePool, Box<dyn std::error::Error>> {
    // Ensure app data directory exists
    let app_data_dir = app_handle.path().app_data_dir().expect("Failed to get app data dir");
    if !app_data_dir.exists() {
        std::fs::create_dir_all(&app_data_dir)?;
    }

    let db_path = app_data_dir.join("live_order.db");
    let db_url = format!("sqlite:{}", db_path.to_string_lossy());

    tracing::info!("Initializing DB at: {}", db_url);

    let options = SqliteConnectOptions::from_str(&db_url)?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .pragma("foreign_keys", "ON");

    let pool_result = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options.clone())
        .await;

    let pool = match pool_result {
        Ok(p) => p,
        Err(e) => {
            tracing::error!("Failed to open DB, possibly corrupted. Error: {}. Attempting backup and recreate...", e);
            let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
            let corrupt_backup_path = app_data_dir.join(format!("live_order_corrupted_{}.db", timestamp));
            
            if db_path.exists() {
                std::fs::rename(&db_path, &corrupt_backup_path)?;
                tracing::info!("Corrupted DB backed up to: {:?}", corrupt_backup_path);
            }
            
            // Try connecting again (which will create a new DB since we moved the old one)
            SqlitePoolOptions::new()
                .max_connections(5)
                .connect_with(options)
                .await?
        }
    };

    tracing::info!("Running migrations...");
    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}

#[tauri::command]
pub async fn backup_db(app_handle: tauri::AppHandle) -> Result<String, String> {
    let app_data_dir = app_handle.path().app_data_dir().map_err(|_| "Failed to get app data dir")?;
    let db_path = app_data_dir.join("live_order.db");
    let backup_dir = app_data_dir.join("backups");
    
    if !backup_dir.exists() {
        std::fs::create_dir_all(&backup_dir).map_err(|e| e.to_string())?;
    }
    
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let backup_path = backup_dir.join(format!("live_order_backup_{}.db", timestamp));
    
    std::fs::copy(&db_path, &backup_path).map_err(|e| e.to_string())?;
    
    tracing::info!("Database backed up to: {:?}", backup_path);
    Ok(backup_path.to_string_lossy().to_string())
}
