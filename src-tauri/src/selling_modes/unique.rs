use crate::parser::models::ParsedIntent;
use crate::events::models::LiveEvent;
use super::models::{OrderClaimResult, ClaimStatus};
use super::engine::{SellingMode, SessionContext};
use uuid::Uuid;

pub struct UniqueMode;

impl SellingMode for UniqueMode {
    fn apply(&self, intent: &ParsedIntent, event: &LiveEvent, ctx: &SessionContext) -> OrderClaimResult {
        // Unique mode usually means 1 item only.
        // We will need a way to check if there is already a winner.
        // If stock_qty > 0, we can grant Won.
        // If stock_qty <= 0, we grant Lost or Waitlist.
        // For a purely memory-based evaluation without DB:
        let stock_qty = if let Some(variant) = &ctx.variant {
            variant.stock_qty
        } else {
            ctx.product.stock_qty
        };

        if stock_qty > 0 {
            OrderClaimResult {
                claim_id: Uuid::new_v4().to_string(),
                session_id: ctx.session_id.clone(),
                event_id: event.id.clone(),
                user_id: event.user_id.clone(),
                sku: intent.sku.clone(),
                quantity: 1, // Unique mode grants 1 regardless of requested quantity?
                status: ClaimStatus::Won,
                message: Some("Won unique item".to_string()),
            }
        } else {
            OrderClaimResult {
                claim_id: Uuid::new_v4().to_string(),
                session_id: ctx.session_id.clone(),
                event_id: event.id.clone(),
                user_id: event.user_id.clone(),
                sku: intent.sku.clone(),
                quantity: 1,
                status: ClaimStatus::Waitlist, // Or Lost
                message: Some("Added to waitlist for unique item".to_string()),
            }
        }
    }
}
