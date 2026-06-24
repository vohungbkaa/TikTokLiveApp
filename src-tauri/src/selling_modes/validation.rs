use crate::parser::models::ParsedIntent;
use super::engine::SessionContext;

pub fn check_pending_info(intent: &ParsedIntent, ctx: &SessionContext) -> bool {
    // Return true if missing required variants
    // Currently a placeholder
    false
}
