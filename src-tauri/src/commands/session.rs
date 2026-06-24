use tauri::{State, Emitter};
use crate::db::DbState;
use crate::sessions::models::{LiveSession, CreateSessionInput};
use crate::sessions::repository::SessionRepository;

#[tauri::command]
pub async fn create_session(
    input: CreateSessionInput,
    state: State<'_, DbState>,
    app_handle: tauri::AppHandle,
) -> Result<LiveSession, String> {
    let repo = SessionRepository::new(&state.pool);
    let session = repo.create(input).await.map_err(|e| e.to_string())?;
    
    let _ = app_handle.emit("session:created", &session);
    Ok(session)
}

#[tauri::command]
pub async fn get_sessions(state: State<'_, DbState>) -> Result<Vec<LiveSession>, String> {
    let repo = SessionRepository::new(&state.pool);
    repo.list().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn start_session(
    id: String,
    state: State<'_, DbState>,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    let repo = SessionRepository::new(&state.pool);
    repo.update_status(&id, "running").await.map_err(|e| e.to_string())?;
    
    let _ = app_handle.emit("session:started", &id);
    Ok(())
}

#[tauri::command]
pub async fn end_session(
    id: String,
    state: State<'_, DbState>,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    let repo = SessionRepository::new(&state.pool);
    repo.update_status(&id, "ended").await.map_err(|e| e.to_string())?;
    
    let _ = app_handle.emit("session:ended", &id);
    Ok(())
}

#[tauri::command]
pub async fn open_session_history(
    id: String,
    state: State<'_, DbState>,
) -> Result<Option<LiveSession>, String> {
    let repo = SessionRepository::new(&state.pool);
    repo.get(&id).await.map_err(|e| e.to_string())
}

#[derive(serde::Deserialize)]
pub struct SessionProductInput {
    pub product_id: String,
    pub initial_stock_qty: i32,
    pub price: f64,
    pub selling_mode: String,
}

#[tauri::command]
pub async fn set_session_products(
    session_id: String,
    products: Vec<SessionProductInput>,
    state: State<'_, DbState>,
) -> Result<(), String> {
    let mut tx = state.pool.begin().await.map_err(|e| e.to_string())?;
    
    for p in products {
        sqlx::query(
            "INSERT INTO session_products (session_id, product_id, initial_stock_qty, current_stock_qty, price, selling_mode) 
             VALUES (?, ?, ?, ?, ?, ?)
             ON CONFLICT (session_id, product_id) DO UPDATE SET
             initial_stock_qty = excluded.initial_stock_qty,
             current_stock_qty = excluded.current_stock_qty,
             price = excluded.price,
             selling_mode = excluded.selling_mode"
        )
        .bind(&session_id)
        .bind(&p.product_id)
        .bind(p.initial_stock_qty)
        .bind(p.initial_stock_qty)
        .bind(p.price)
        .bind(&p.selling_mode)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;
    }
    
    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(())
}
