use tauri::State;
use crate::db::DbState;
use crate::parser::models::ParsedIntent;
use crate::parser::service::ParserService;
use crate::products::repository::ProductRepository;

#[tauri::command]
pub async fn debug_parse_comment(
    comment: String,
    state: State<'_, DbState>,
) -> Result<ParsedIntent, String> {
    let repo = ProductRepository::new(&state.pool);
    let active_products = repo.list().await.map_err(|e| e.to_string())?; // Should ideally be active only
    
    Ok(ParserService::parse_comment(&comment, &active_products))
}
