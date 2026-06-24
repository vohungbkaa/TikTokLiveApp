use tauri::{State, command};
use crate::db::DbState;
use crate::orders::models::Order;
use crate::orders::repository::OrderRepository;
use crate::orders::summary::{OrderSummary, get_session_summary};

#[command]
pub async fn list_orders_by_session(
    session_id: String,
    state: State<'_, DbState>,
) -> Result<Vec<Order>, String> {
    OrderRepository::list_orders_by_session(&state.pool, &session_id)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn get_order_summary(
    session_id: String,
    state: State<'_, DbState>,
) -> Result<OrderSummary, String> {
    get_session_summary(&state.pool, &session_id)
        .await
        .map_err(|e| e.to_string())
}
