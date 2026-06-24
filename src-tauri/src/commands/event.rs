use tauri::State;
use crate::db::DbState;
use crate::events::models::{IncomingEvent, LiveEvent};
use crate::events::ingestion::EventIngestionService;
use crate::events::repository::EventRepository;

#[tauri::command]
pub async fn get_session_events(
    session_id: String,
    limit: Option<i64>,
    state: State<'_, DbState>,
) -> Result<Vec<LiveEvent>, String> {
    let repo = EventRepository::new(&state.pool);
    let mut events = repo
        .list_events_by_session(&session_id, limit.unwrap_or(500))
        .await
        .map_err(|e| e.to_string())?;
    events.reverse();
    Ok(events)
}

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
