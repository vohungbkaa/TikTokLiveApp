use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum IntentType {
    Buy,
    Cancel,
    Question,
    Noise,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedIntent {
    pub intent: IntentType,
    pub sku: Option<String>,
    pub quantity: i32,
    pub variants: Vec<String>,
    pub confidence_score: f32,
    pub needs_review: bool,
    pub normalized_text: String,
    pub matched_rules: Vec<String>,
}
