use sqlx::{SqlitePool, Error};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderSummary {
    pub total_orders: i64,
    pub total_revenue: f64,
    pub pending_orders: i64,
}

pub async fn get_session_summary(pool: &SqlitePool, session_id: &str) -> Result<OrderSummary, Error> {
    let total_orders = sqlx::query_scalar::<_, i64>(
        r#"SELECT COUNT(id) FROM orders WHERE session_id = ?"#
    )
    .bind(session_id)
    .fetch_one(pool)
    .await?;

    let total_revenue: f64 = sqlx::query_scalar::<_, f64>(
        r#"SELECT COALESCE(SUM(total_amount), 0.0) FROM orders WHERE session_id = ?"#
    )
    .bind(session_id)
    .fetch_one(pool)
    .await?;

    let pending_orders = sqlx::query_scalar::<_, i64>(
        r#"SELECT COUNT(id) FROM orders WHERE session_id = ? AND status IN ('draft', 'pending_info')"#
    )
    .bind(session_id)
    .fetch_one(pool)
    .await?;

    Ok(OrderSummary {
        total_orders,
        total_revenue,
        pending_orders,
    })
}
