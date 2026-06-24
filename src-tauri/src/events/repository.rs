use sqlx::SqlitePool;
use super::models::LiveEvent;

pub struct EventRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> EventRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn insert_event(&self, event: &LiveEvent) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO live_events (id, session_id, sequence_no, source, platform_event_id, user_id, unique_id, display_name, comment, ts_platform, ts_received, raw_json)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&event.id)
        .bind(&event.session_id)
        .bind(event.sequence_no)
        .bind(&event.source)
        .bind(&event.platform_event_id)
        .bind(&event.user_id)
        .bind(&event.unique_id)
        .bind(&event.display_name)
        .bind(&event.comment)
        .bind(event.ts_platform)
        .bind(event.ts_received)
        .bind(&event.raw_json)
        .execute(self.pool)
        .await?;
        
        Ok(())
    }

    pub async fn get_next_sequence_no(&self, session_id: &str) -> Result<i32, sqlx::Error> {
        let result: Option<i32> = sqlx::query_scalar("SELECT MAX(sequence_no) FROM live_events WHERE session_id = ?")
            .bind(session_id)
            .fetch_optional(self.pool)
            .await?;
            
        Ok(result.unwrap_or(0) + 1)
    }

    pub async fn list_events_by_session(&self, session_id: &str, limit: i64) -> Result<Vec<LiveEvent>, sqlx::Error> {
        sqlx::query_as::<_, LiveEvent>(
            "SELECT id, session_id, sequence_no, source, platform_event_id, user_id, unique_id, display_name, comment, ts_platform, ts_received, raw_json FROM live_events WHERE session_id = ? ORDER BY sequence_no DESC LIMIT ?"
        )
        .bind(session_id)
        .bind(limit)
        .fetch_all(self.pool)
        .await
    }
}
