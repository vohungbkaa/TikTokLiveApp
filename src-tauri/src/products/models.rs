use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    pub id: String,
    pub sku: String,
    pub name: String,
    pub price: f64,
    pub selling_mode: String,
    pub stock_qty: i32,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ProductVariant {
    pub id: String,
    pub product_id: String,
    pub sku: String,
    pub size: Option<String>,
    pub color: Option<String>,
    pub stock_qty: i32,
    pub price: f64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProductInput {
    pub sku: String,
    pub name: String,
    pub price: f64,
    pub selling_mode: String,
    pub stock_qty: i32,
}
