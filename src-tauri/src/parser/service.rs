use crate::products::models::Product;
use super::models::ParsedIntent;
use super::{normalizer, keywords, sku_extractor, quantity, variant, confidence};

pub struct ParserService;

impl ParserService {
    pub fn parse_comment(comment: &str, active_products: &[Product]) -> ParsedIntent {
        let normalized = normalizer::normalize(comment);
        let (intent, rules) = keywords::determine_intent(comment, &normalized);
        let sku = sku_extractor::extract_sku(&normalized, active_products);
        let qty = quantity::extract_quantity(&normalized);
        let variants = variant::extract_variants(&normalized);
        
        let parsed = ParsedIntent {
            intent,
            sku,
            quantity: qty,
            variants,
            confidence_score: 0.0,
            needs_review: false,
            normalized_text: normalized,
            matched_rules: rules,
        };
        
        confidence::calculate_confidence(parsed)
    }
}
