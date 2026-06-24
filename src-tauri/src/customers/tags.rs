use sqlx::{SqlitePool, Error};

pub struct CustomerTags;

impl CustomerTags {
    pub async fn add_tag(pool: &SqlitePool, customer_id: &str, tag: &str) -> Result<(), Error> {
        let now = chrono::Utc::now();
        sqlx::query(
            "INSERT OR IGNORE INTO customer_tags (customer_id, tag, created_at) VALUES (?, ?, ?)"
        )
        .bind(customer_id)
        .bind(tag)
        .bind(now)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn remove_tag(pool: &SqlitePool, customer_id: &str, tag: &str) -> Result<(), Error> {
        sqlx::query(
            "DELETE FROM customer_tags WHERE customer_id = ? AND tag = ?"
        )
        .bind(customer_id)
        .bind(tag)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn get_tags(pool: &SqlitePool, customer_id: &str) -> Result<Vec<String>, Error> {
        let tags = sqlx::query_scalar::<_, String>(
            "SELECT tag FROM customer_tags WHERE customer_id = ?"
        )
        .bind(customer_id)
        .fetch_all(pool)
        .await?;
        Ok(tags)
    }
}
