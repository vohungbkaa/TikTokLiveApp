#[tauri::command]
pub fn app_health_check() -> serde_json::Value {
    serde_json::json!({
        "ok": true,
        "version": env!("CARGO_PKG_VERSION")
    })
}
