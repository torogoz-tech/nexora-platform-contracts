use crate::ApprovalRequestTarget;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ApprovalRequestCreate {
    pub target: ApprovalRequestTarget,
    pub reason: String,
    pub expires_in_seconds: u32,
}
