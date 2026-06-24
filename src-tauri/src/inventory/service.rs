use sqlx::SqlitePool;
use crate::products::repository::ProductRepository;

pub struct InventoryService<'a> {
    pool: &'a SqlitePool,
}

impl<'a> InventoryService<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn check_stock(&self, sku: &str) -> Result<Option<i32>, sqlx::Error> {
        let repo = ProductRepository::new(self.pool);
        if let Some(product) = repo.get_by_sku(sku).await? {
            Ok(Some(product.stock_qty))
        } else {
            Ok(None)
        }
    }

    pub async fn reserve_stock(&self, product_id: &str, qty: i32) -> Result<bool, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        
        let row = sqlx::query("SELECT stock_qty FROM products WHERE id = ?")
            .bind(product_id)
            .fetch_optional(&mut *tx)
            .await?;
            
        if let Some(r) = row {
            use sqlx::Row;
            let current_stock: i32 = r.try_get("stock_qty")?;
            if current_stock >= qty {
                sqlx::query("UPDATE products SET stock_qty = stock_qty - ?, updated_at = ? WHERE id = ?")
                    .bind(qty)
                    .bind(chrono::Utc::now())
                    .bind(product_id)
                    .execute(&mut *tx)
                    .await?;
                tx.commit().await?;
                return Ok(true);
            }
        }
        tx.rollback().await?;
        Ok(false)
    }

    pub async fn release_stock(&self, product_id: &str, qty: i32) -> Result<(), sqlx::Error> {
        let repo = ProductRepository::new(self.pool);
        repo.update_stock(product_id, qty).await
    }
}
