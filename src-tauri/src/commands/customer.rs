use tauri::{State, command};
use crate::db::DbState;
use crate::customers::models::Customer;

#[command]
pub async fn search_customers(
    query: String,
    state: State<'_, DbState>,
) -> Result<Vec<Customer>, String> {
    let search_pattern = format!("%{}%", query);
    let customers = sqlx::query_as::<_, Customer>(
        r#"
        SELECT id, platform_user_id, unique_id, display_name, phone, address, note, created_at
        FROM customers
        WHERE display_name LIKE ? OR unique_id LIKE ? OR phone LIKE ?
        LIMIT 20
        "#
    )
    .bind(&search_pattern)
    .bind(&search_pattern)
    .bind(&search_pattern)
    .fetch_all(&state.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(customers)
}

#[command]
pub async fn update_customer_profile(
    id: String,
    phone: Option<String>,
    address: Option<String>,
    note: Option<String>,
    state: State<'_, DbState>,
) -> Result<(), String> {
    sqlx::query(
        "UPDATE customers SET phone = ?, address = ?, note = ? WHERE id = ?"
    )
    .bind(phone)
    .bind(address)
    .bind(note)
    .bind(id)
    .execute(&state.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}
