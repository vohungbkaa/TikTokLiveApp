use sqlx::{Sqlite, Transaction};

pub trait Repository {
    // Basic repository marker
}

pub struct SessionRepository;
pub struct EventRepository;
pub struct ProductRepository;
pub struct CustomerRepository;
pub struct OrderRepository;
pub struct SettingsRepository;

/// Helper to execute a pipeline within a transaction
pub async fn run_in_transaction<F, Fut, T, E>(
    pool: &sqlx::SqlitePool,
    operation: F,
) -> Result<T, E>
where
    F: FnOnce(Transaction<'static, Sqlite>) -> Fut,
    Fut: std::future::Future<Output = Result<(T, Transaction<'static, Sqlite>), E>>,
    E: From<sqlx::Error>,
{
    let mut tx = pool.begin().await?;
    let (result, tx) = operation(tx).await?;
    tx.commit().await?;
    Ok(result)
}
