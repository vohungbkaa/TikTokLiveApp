use tauri::State;
use crate::db::DbState;
use crate::events::models::{IncomingEvent, LiveEvent};
use crate::events::ingestion::EventIngestionService;

#[tauri::command]
pub async fn test_ingest_event(
    session_id: String,
    comment: String,
    state: State<'_, DbState>,
    app_handle: tauri::AppHandle,
) -> Result<LiveEvent, String> {
    let service = EventIngestionService::new(state.pool.clone(), app_handle);
    let incoming = IncomingEvent {
        source: "mock".to_string(),
        platform_event_id: None,
        user_id: Some("mock_user_123".to_string()),
        unique_id: Some("mock_user".to_string()),
        display_name: Some("Khách Giả Lập".to_string()),
        comment: Some(comment),
        ts_platform: Some(chrono::Utc::now()),
        raw_json: None,
    };
    
    service.ingest(&session_id, incoming).await
}
