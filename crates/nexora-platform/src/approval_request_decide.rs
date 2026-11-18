use crate::ApprovalDecision;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ApprovalRequestDecide {
    pub expected_revision: i64,
    pub decision: ApprovalDecision,
    pub reason: String,
    pub approval_expires_in_seconds: u32,
}
