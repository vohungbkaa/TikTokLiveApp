use sqlx::{Error, Sqlite, Transaction};
use uuid::Uuid;

pub struct CustomerIdentityResolver;

impl CustomerIdentityResolver {
    pub async fn resolve<'a>(
        tx: &mut Transaction<'a, Sqlite>,
        user_id: Option<&str>,
        unique_id: Option<&str>,
        display_name: Option<&str>
    ) -> Result<String, Error> {
        if let Some(uid) = user_id {
            if let Some(id) = sqlx::query_scalar::<_, String>("SELECT id FROM customers WHERE platform_user_id = ?").bind(uid).fetch_optional(&mut **tx).await? {
                return Ok(id);
            }
        }

        if let Some(unq) = unique_id {
            if let Some(id) = sqlx::query_scalar::<_, String>("SELECT id FROM customers WHERE unique_id = ?").bind(unq).fetch_optional(&mut **tx).await? {
                if let Some(uid) = user_id {
                    sqlx::query("UPDATE customers SET platform_user_id = ? WHERE id = ?").bind(uid).bind(&id).execute(&mut **tx).await?;
                }
                return Ok(id);
            }
        }

        if let Some(name) = display_name {
            if user_id.is_none() && unique_id.is_none() {
                if let Some(id) = sqlx::query_scalar::<_, String>("SELECT id FROM customers WHERE display_name = ? AND platform_user_id IS NULL AND unique_id IS NULL").bind(name).fetch_optional(&mut **tx).await? {
                    return Ok(id);
                }
            }
        }

        let new_id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now();
        sqlx::query(
            "INSERT INTO customers (id, platform_user_id, unique_id, display_name, created_at) VALUES (?, ?, ?, ?, ?)"
        )
        .bind(&new_id)
        .bind(user_id)
        .bind(unique_id)
        .bind(display_name)
        .bind(now)
        .execute(&mut **tx)
        .await?;

        Ok(new_id)
    }
}
