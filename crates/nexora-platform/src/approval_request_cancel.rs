use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ApprovalRequestCancel {
    pub expected_revision: i64,
    pub reason: String,
}
