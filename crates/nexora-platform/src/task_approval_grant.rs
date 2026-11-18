use crate::TaskTransitionRequest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskApprovalGrant {
    pub transition_id: String,
    pub parameters: TaskTransitionRequest,
    pub reason: String,
    pub expires_in_seconds: u32,
}
