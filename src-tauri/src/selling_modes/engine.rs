use crate::parser::models::{ParsedIntent, IntentType};
use crate::events::models::LiveEvent;
use crate::products::models::{Product, ProductVariant};
use super::models::{OrderClaimResult, ClaimStatus};

pub struct SessionContext {
    pub session_id: String,
    pub product: Product,
    pub variant: Option<ProductVariant>,
}

pub trait SellingMode {
    fn apply(&self, intent: &ParsedIntent, event: &LiveEvent, ctx: &SessionContext) -> OrderClaimResult;
}

pub struct SellingModeEngine;

impl SellingModeEngine {
    pub fn apply(intent: &ParsedIntent, event: &LiveEvent, ctx: &SessionContext) -> OrderClaimResult {
        if intent.intent != IntentType::Buy {
            return OrderClaimResult {
                claim_id: uuid::Uuid::new_v4().to_string(),
                session_id: ctx.session_id.clone(),
                event_id: event.id.clone(),
                user_id: event.user_id.clone(),
                sku: intent.sku.clone(),
                quantity: intent.quantity,
                status: ClaimStatus::Ignored,
                message: Some("Not a buy intent".to_string()),
            };
        }

        if intent.needs_review {
            return OrderClaimResult {
                claim_id: uuid::Uuid::new_v4().to_string(),
                session_id: ctx.session_id.clone(),
                event_id: event.id.clone(),
                user_id: event.user_id.clone(),
                sku: intent.sku.clone(),
                quantity: intent.quantity,
                status: ClaimStatus::NeedsReview,
                message: Some("Parser needs review".to_string()),
            };
        }

        match ctx.product.selling_mode.as_str() {
            "stock" => crate::selling_modes::stock::StockMode.apply(intent, event, ctx),
            "unique" => crate::selling_modes::unique::UniqueMode.apply(intent, event, ctx),
            "preorder" => crate::selling_modes::preorder::PreorderMode.apply(intent, event, ctx),
            _ => OrderClaimResult {
                claim_id: uuid::Uuid::new_v4().to_string(),
                session_id: ctx.session_id.clone(),
                event_id: event.id.clone(),
                user_id: event.user_id.clone(),
                sku: intent.sku.clone(),
                quantity: intent.quantity,
                status: ClaimStatus::Ignored,
                message: Some(format!("Unknown selling mode: {}", ctx.product.selling_mode)),
            }
        }
    }
}
