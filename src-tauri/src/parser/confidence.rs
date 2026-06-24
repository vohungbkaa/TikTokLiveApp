use super::models::{IntentType, ParsedIntent};

pub fn calculate_confidence(mut intent: ParsedIntent) -> ParsedIntent {
    let mut score: f32 = 0.0;
    
    if intent.sku.is_some() {
        score += 0.6;
        if intent.intent == IntentType::Buy {
            score += 0.3;
        } else if intent.intent == IntentType::Noise {
            intent.intent = IntentType::Buy;
            intent.matched_rules.push("implicit_buy_sku".to_string());
            score += 0.2;
        }
    } else {
        if intent.intent == IntentType::Buy {
            score += 0.2;
            intent.needs_review = true;
            intent.matched_rules.push("missing_sku".to_string());
        }
    }
    
    if !intent.variants.is_empty() {
        score += 0.1;
    }
    
    intent.confidence_score = score.min(1.0);
    
    if intent.confidence_score < 0.8 && intent.intent == IntentType::Buy {
        intent.needs_review = true;
    }
    
    intent
}
