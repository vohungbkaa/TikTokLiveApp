use sqlx::{SqlitePool, Error, Sqlite, Transaction};
use super::models::Customer;

pub struct CustomerRepository;

impl CustomerRepository {
    pub async fn ensure_customer<'a>(tx: &mut Transaction<'a, Sqlite>, user_id: &str, display_name: Option<String>) -> Result<String, Error> {
        let existing = sqlx::query_scalar::<_, String>(
            "SELECT id FROM customers WHERE platform_user_id = ?"
        )
        .bind(user_id)
        .fetch_optional(&mut **tx)
        .await?;

        if let Some(id) = existing {
            return Ok(id);
        }

        let new_id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now();
        sqlx::query(
            "INSERT INTO customers (id, platform_user_id, display_name, created_at) VALUES (?, ?, ?, ?)"
        )
        .bind(&new_id)
        .bind(user_id)
        .bind(display_name)
        .bind(now)
        .execute(&mut **tx)
        .await?;

        Ok(new_id)
    }
}
