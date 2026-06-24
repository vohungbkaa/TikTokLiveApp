use super::models::IntentType;
use super::service::ParserService;
use crate::products::models::Product;

#[test]
fn test_buy_exact_sku() {
    let products = vec![
        Product {
            id: "1".to_string(),
            sku: "A12".to_string(),
            name: "Test".to_string(),
            price: 100.0,
            selling_mode: "stock".to_string(),
            stock_qty: 10,
            is_active: true,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    ];
    
    let result = ParserService::parse_comment("chốt a12 2 cái màu đỏ size m", &products);
    
    assert_eq!(result.intent, IntentType::Buy);
    assert_eq!(result.sku.unwrap(), "A12");
    assert_eq!(result.quantity, 2);
    assert!(result.variants.contains(&"do".to_string()));
    assert!(result.variants.contains(&"SIZE M".to_string()));
    assert!(!result.needs_review);
}

#[test]
fn test_implicit_buy() {
    let products = vec![
        Product {
            id: "1".to_string(),
            sku: "B34".to_string(),
            name: "Test".to_string(),
            price: 100.0,
            selling_mode: "stock".to_string(),
            stock_qty: 10,
            is_active: true,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    ];
    
    let result = ParserService::parse_comment("b 34", &products);
    assert_eq!(result.intent, IntentType::Buy);
    assert_eq!(result.sku.unwrap(), "B34");
    assert_eq!(result.quantity, 1);
}

#[test]
fn test_missing_sku_review() {
    let products = vec![];
    let result = ParserService::parse_comment("cho mình chốt 1 cái đỏ nhé", &products);
    assert_eq!(result.intent, IntentType::Buy);
    assert!(result.sku.is_none());
    assert!(result.needs_review);
}
