#[cfg(test)]
mod tests {
    use super::super::engine::{SellingModeEngine, SessionContext};
    use crate::parser::models::{ParsedIntent, IntentType};
    use crate::events::models::LiveEvent;
    use crate::products::models::Product;
    use super::super::models::ClaimStatus;
    use chrono::Utc;

    fn mock_product(mode: &str, stock: i32) -> Product {
        Product {
            id: "p1".to_string(),
            sku: "SKU1".to_string(),
            name: "Test".to_string(),
            price: 100.0,
            selling_mode: mode.to_string(),
            stock_qty: stock,
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn mock_event() -> LiveEvent {
        LiveEvent {
            id: "e1".to_string(),
            session_id: "s1".to_string(),
            sequence_no: 1,
            source: "tiktok".to_string(),
            platform_event_id: Some("pe1".to_string()),
            user_id: Some("u1".to_string()),
            unique_id: Some("u1".to_string()),
            display_name: Some("User 1".to_string()),
            comment: Some("chot SKU1".to_string()),
            ts_platform: Some(Utc::now()),
            ts_received: Utc::now(),
            raw_json: None,
        }
    }

    fn mock_intent() -> ParsedIntent {
        ParsedIntent {
            intent: IntentType::Buy,
            sku: Some("SKU1".to_string()),
            quantity: 1,
            variants: vec![],
            confidence_score: 1.0,
            needs_review: false,
            normalized_text: "chot sku1".to_string(),
            matched_rules: vec![],
        }
    }

    #[test]
    fn test_stock_mode_confirmed() {
        let product = mock_product("stock", 10);
        let event = mock_event();
        let intent = mock_intent();
        let ctx = SessionContext {
            session_id: "s1".to_string(),
            product,
            variant: None,
        };

        let result = SellingModeEngine::apply(&intent, &event, &ctx);
        assert_eq!(result.status, ClaimStatus::Confirmed);
    }

    #[test]
    fn test_stock_mode_out_of_stock() {
        let product = mock_product("stock", 0);
        let event = mock_event();
        let intent = mock_intent();
        let ctx = SessionContext {
            session_id: "s1".to_string(),
            product,
            variant: None,
        };

        let result = SellingModeEngine::apply(&intent, &event, &ctx);
        assert_eq!(result.status, ClaimStatus::OutOfStock);
    }

    #[test]
    fn test_unique_mode_won() {
        let product = mock_product("unique", 1);
        let event = mock_event();
        let intent = mock_intent();
        let ctx = SessionContext {
            session_id: "s1".to_string(),
            product,
            variant: None,
        };

        let result = SellingModeEngine::apply(&intent, &event, &ctx);
        assert_eq!(result.status, ClaimStatus::Won);
    }
}
