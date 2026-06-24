use tauri::State;
use crate::db::DbState;
use crate::products::models::{Product, CreateProductInput};
use crate::products::repository::ProductRepository;

#[tauri::command]
pub async fn list_products(state: State<'_, DbState>) -> Result<Vec<Product>, String> {
    let repo = ProductRepository::new(&state.pool);
    repo.list().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_product(
    input: CreateProductInput,
    state: State<'_, DbState>,
) -> Result<Product, String> {
    let repo = ProductRepository::new(&state.pool);
    repo.create(input).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_stock(
    id: String,
    delta: i32,
    state: State<'_, DbState>,
) -> Result<(), String> {
    let repo = ProductRepository::new(&state.pool);
    repo.update_stock(&id, delta).await.map_err(|e| e.to_string())
}
