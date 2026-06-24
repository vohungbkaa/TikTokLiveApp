use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct LiveEvent {
    pub id: String,
    pub session_id: String,
    pub sequence_no: i32,
    pub source: String,
    pub platform_event_id: Option<String>,
    pub user_id: Option<String>,
    pub unique_id: Option<String>,
    pub display_name: Option<String>,
    pub comment: Option<String>,
    pub ts_platform: Option<chrono::DateTime<chrono::Utc>>,
    pub ts_received: chrono::DateTime<chrono::Utc>,
    pub raw_json: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IncomingEvent {
    pub source: String,
    pub platform_event_id: Option<String>,
    pub user_id: Option<String>,
    pub unique_id: Option<String>,
    pub display_name: Option<String>,
    pub comment: Option<String>,
    pub ts_platform: Option<chrono::DateTime<chrono::Utc>>,
    pub raw_json: Option<String>,
}
