use sqlx::SqlitePool;
use uuid::Uuid;
use super::models::{LiveSession, CreateSessionInput};

pub struct SessionRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> SessionRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: CreateSessionInput) -> Result<LiveSession, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now();
        
        sqlx::query(
            r#"
            INSERT INTO live_sessions (id, platform, platform_session_id, title, status, created_at)
            VALUES (?, ?, ?, ?, 'draft', ?)
            "#
        )
        .bind(&id)
        .bind(&input.platform)
        .bind(&input.platform_session_id)
        .bind(&input.title)
        .bind(now)
        .execute(self.pool)
        .await?;
        
        Ok(LiveSession {
            id,
            platform: input.platform,
            platform_session_id: input.platform_session_id,
            title: input.title,
            status: "draft".to_string(),
            started_at: None,
            ended_at: None,
            created_at: now,
        })
    }

    pub async fn get(&self, id: &str) -> Result<Option<LiveSession>, sqlx::Error> {
        let session = sqlx::query_as::<_, LiveSession>(
            r#"
            SELECT id, platform, platform_session_id, title, status, started_at, ended_at, created_at
            FROM live_sessions WHERE id = ?
            "#
        )
        .bind(id)
        .fetch_optional(self.pool)
        .await?;
        Ok(session)
    }

    pub async fn list(&self) -> Result<Vec<LiveSession>, sqlx::Error> {
        let sessions = sqlx::query_as::<_, LiveSession>(
            r#"
            SELECT id, platform, platform_session_id, title, status, started_at, ended_at, created_at
            FROM live_sessions
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(self.pool)
        .await?;
        Ok(sessions)
    }
    
    pub async fn update_status(&self, id: &str, status: &str) -> Result<(), sqlx::Error> {
        let now = chrono::Utc::now();
        if status == "running" {
            sqlx::query(
                "UPDATE live_sessions SET status = ?, started_at = COALESCE(started_at, ?) WHERE id = ?"
            )
            .bind(status)
            .bind(now)
            .bind(id)
            .execute(self.pool).await?;
        } else if status == "ended" {
            sqlx::query(
                "UPDATE live_sessions SET status = ?, ended_at = ? WHERE id = ?"
            )
            .bind(status)
            .bind(now)
            .bind(id)
            .execute(self.pool).await?;
        } else {
            sqlx::query(
                "UPDATE live_sessions SET status = ? WHERE id = ?"
            )
            .bind(status)
            .bind(id)
            .execute(self.pool).await?;
        }
        Ok(())
    }
}
