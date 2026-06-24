use sqlx::SqlitePool;
use super::models::Product;

pub struct SkuResolver<'a> {
    pool: &'a SqlitePool,
}

impl<'a> SkuResolver<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn resolve(&self, text: &str) -> Result<Option<Product>, sqlx::Error> {
        let text_clean = text.trim().to_lowercase();
        let text_no_space = text_clean.replace(" ", "");
        
        let products = sqlx::query_as::<_, Product>(
            "SELECT id, sku, name, price, selling_mode, stock_qty, is_active, created_at, updated_at FROM products WHERE is_active = 1"
        )
        .fetch_all(self.pool)
        .await?;
        
        for p in products {
            let sku_clean = p.sku.to_lowercase().replace(" ", "");
            if sku_clean == text_no_space {
                return Ok(Some(p));
            }
        }
        
        Ok(None)
    }
}
