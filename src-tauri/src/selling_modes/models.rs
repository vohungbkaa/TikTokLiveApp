use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ClaimStatus {
    Confirmed,
    OutOfStock,
    Won,
    Lost,
    Waitlist,
    PendingInfo,
    NeedsReview,
    Ignored,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderClaimResult {
    pub claim_id: String,
    pub session_id: String,
    pub event_id: String,
    pub user_id: Option<String>,
    pub sku: Option<String>,
    pub quantity: i32,
    pub status: ClaimStatus,
    pub message: Option<String>,
}
