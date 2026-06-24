use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Customer {
    pub id: String,
    pub platform_user_id: Option<String>,
    pub unique_id: Option<String>,
    pub display_name: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub note: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct CustomerTag {
    pub customer_id: String,
    pub tag: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct CustomerRisk {
    pub id: String,
    pub customer_id: String,
    pub risk_type: String,
    pub reason: Option<String>,
    pub evidence: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
