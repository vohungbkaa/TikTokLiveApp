use sqlx::SqlitePool;
use uuid::Uuid;
use tauri::Emitter;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::models::{IncomingEvent, LiveEvent};
use super::repository::EventRepository;
use super::dedupe::Deduplicator;

#[derive(Clone)]
pub struct EventIngestionService {
    pool: SqlitePool,
    app_handle: tauri::AppHandle,
    deduper: Arc<Mutex<Deduplicator>>,
}

impl EventIngestionService {
    pub fn new(pool: SqlitePool, app_handle: tauri::AppHandle) -> Self {
        Self {
            pool,
            app_handle,
            deduper: Arc::new(Mutex::new(Deduplicator::new(10))), // 10s dedupe window
        }
    }

    pub async fn ingest(&self, session_id: &str, incoming: IncomingEvent) -> Result<LiveEvent, String> {
        let dedupe_key = if let Some(ref pid) = incoming.platform_event_id {
            format!("pid:{}", pid)
        } else {
            format!("content:{}:{}:{:?}", incoming.user_id.as_deref().unwrap_or(""), incoming.comment.as_deref().unwrap_or(""), incoming.ts_platform)
        };
        
        let mut deduper = self.deduper.lock().await;
        if deduper.is_duplicate(&dedupe_key) {
            return Err("Duplicate event".to_string());
        }
        drop(deduper);
        
        let repo = EventRepository::new(&self.pool);
        let seq_no = repo.get_next_sequence_no(session_id).await.map_err(|e| e.to_string())?;
        
        let event = LiveEvent {
            id: Uuid::new_v4().to_string(),
            session_id: session_id.to_string(),
            sequence_no: seq_no,
            source: incoming.source,
            platform_event_id: incoming.platform_event_id,
            user_id: incoming.user_id,
            unique_id: incoming.unique_id,
            display_name: incoming.display_name,
            comment: incoming.comment,
            ts_platform: incoming.ts_platform,
            ts_received: chrono::Utc::now(),
            raw_json: incoming.raw_json,
        };
        
        repo.insert_event(&event).await.map_err(|e| e.to_string())?;
        
        // Emit to UI
        let _ = self.app_handle.emit("live_event_received", &event);
        
        Ok(event)
    }
}
