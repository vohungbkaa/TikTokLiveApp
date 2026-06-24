use crate::parser::models::ParsedIntent;
use crate::events::models::LiveEvent;
use super::models::{OrderClaimResult, ClaimStatus};
use super::engine::{SellingMode, SessionContext};
use uuid::Uuid;

pub struct PreorderMode;

impl SellingMode for PreorderMode {
    fn apply(&self, intent: &ParsedIntent, event: &LiveEvent, ctx: &SessionContext) -> OrderClaimResult {
        let quota = if let Some(variant) = &ctx.variant {
            variant.stock_qty
        } else {
            ctx.product.stock_qty
        };

        if quota >= intent.quantity {
            OrderClaimResult {
                claim_id: Uuid::new_v4().to_string(),
                session_id: ctx.session_id.clone(),
                event_id: event.id.clone(),
                user_id: event.user_id.clone(),
                sku: intent.sku.clone(),
                quantity: intent.quantity,
                status: ClaimStatus::Confirmed,
                message: Some("Preorder confirmed".to_string()),
            }
        } else {
            OrderClaimResult {
                claim_id: Uuid::new_v4().to_string(),
                session_id: ctx.session_id.clone(),
                event_id: event.id.clone(),
                user_id: event.user_id.clone(),
                sku: intent.sku.clone(),
                quantity: intent.quantity,
                status: ClaimStatus::Waitlist,
                message: Some("Preorder waitlist (quota exceeded)".to_string()),
            }
        }
    }
}
