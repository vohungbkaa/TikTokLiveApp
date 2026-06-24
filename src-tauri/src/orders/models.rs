use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub id: String,
    pub session_id: String,
    pub customer_id: String,
    pub status: String,
    pub total_amount: f64,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct OrderItem {
    pub id: String,
    pub order_id: String,
    pub claim_id: Option<String>,
    pub sku: String,
    pub variant_id: Option<String>,
    pub qty: i32,
    pub price: f64,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct OrderClaim {
    pub id: String,
    pub session_id: String,
    pub event_id: String,
    pub customer_id: String,
    pub sku: String,
    pub variant_id: Option<String>,
    pub qty: i32,
    pub confidence: Option<f64>,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
