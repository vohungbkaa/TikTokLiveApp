use sqlx::SqlitePool;
use uuid::Uuid;
use super::models::{Product, CreateProductInput};

pub struct ProductRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> ProductRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: CreateProductInput) -> Result<Product, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now();
        
        sqlx::query(
            r#"
            INSERT INTO products (id, sku, name, price, selling_mode, stock_qty, is_active, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, 1, ?, ?)
            "#
        )
        .bind(&id)
        .bind(&input.sku)
        .bind(&input.name)
        .bind(input.price)
        .bind(&input.selling_mode)
        .bind(input.stock_qty)
        .bind(now)
        .bind(now)
        .execute(self.pool)
        .await?;
        
        Ok(Product {
            id,
            sku: input.sku,
            name: input.name,
            price: input.price,
            selling_mode: input.selling_mode,
            stock_qty: input.stock_qty,
            is_active: true,
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn list(&self) -> Result<Vec<Product>, sqlx::Error> {
        sqlx::query_as::<_, Product>(
            "SELECT id, sku, name, price, selling_mode, stock_qty, is_active, created_at, updated_at FROM products WHERE is_active = 1 ORDER BY created_at DESC"
        )
        .fetch_all(self.pool)
        .await
    }

    pub async fn get_by_sku(&self, sku: &str) -> Result<Option<Product>, sqlx::Error> {
        sqlx::query_as::<_, Product>(
            "SELECT id, sku, name, price, selling_mode, stock_qty, is_active, created_at, updated_at FROM products WHERE sku = ?"
        )
        .bind(sku)
        .fetch_optional(self.pool)
        .await
    }

    pub async fn update_stock(&self, id: &str, delta: i32) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE products SET stock_qty = stock_qty + ?, updated_at = ? WHERE id = ?")
            .bind(delta)
            .bind(chrono::Utc::now())
            .bind(id)
            .execute(self.pool)
            .await?;
        Ok(())
    }
}
