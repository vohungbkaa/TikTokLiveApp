use sqlx::{SqlitePool, Error, Sqlite, Transaction};
use super::models::{Order, OrderItem};

pub struct OrderRepository;

impl OrderRepository {
    pub async fn create_order<'a>(tx: &mut Transaction<'a, Sqlite>, order: &Order) -> Result<(), Error> {
        sqlx::query(
            r#"
            INSERT INTO orders (id, session_id, customer_id, status, total_amount, created_at)
            VALUES (?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&order.id)
        .bind(&order.session_id)
        .bind(&order.customer_id)
        .bind(&order.status)
        .bind(order.total_amount)
        .bind(order.created_at)
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    pub async fn get_order_by_customer_session<'a>(tx: &mut Transaction<'a, Sqlite>, customer_id: &str, session_id: &str) -> Result<Option<Order>, Error> {
        let order = sqlx::query_as::<_, Order>(
            r#"
            SELECT id, session_id, customer_id, status, total_amount, created_at
            FROM orders
            WHERE customer_id = ? AND session_id = ?
            "#
        )
        .bind(customer_id)
        .bind(session_id)
        .fetch_optional(&mut **tx)
        .await?;
        Ok(order)
    }

    pub async fn create_order_item<'a>(tx: &mut Transaction<'a, Sqlite>, item: &OrderItem) -> Result<(), Error> {
        sqlx::query(
            r#"
            INSERT INTO order_items (id, order_id, claim_id, sku, variant_id, qty, price, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&item.id)
        .bind(&item.order_id)
        .bind(&item.claim_id)
        .bind(&item.sku)
        .bind(&item.variant_id)
        .bind(item.qty)
        .bind(item.price)
        .bind(item.created_at)
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    pub async fn update_order_total<'a>(tx: &mut Transaction<'a, Sqlite>, order_id: &str, total_amount: f64) -> Result<(), Error> {
        sqlx::query(
            "UPDATE orders SET total_amount = ? WHERE id = ?"
        )
        .bind(total_amount)
        .bind(order_id)
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    pub async fn list_orders_by_session(pool: &SqlitePool, session_id: &str) -> Result<Vec<Order>, Error> {
        let orders = sqlx::query_as::<_, Order>(
            r#"
            SELECT id, session_id, customer_id, status, total_amount, created_at
            FROM orders
            WHERE session_id = ?
            ORDER BY created_at DESC
            "#
        )
        .bind(session_id)
        .fetch_all(pool)
        .await?;
        Ok(orders)
    }
}
