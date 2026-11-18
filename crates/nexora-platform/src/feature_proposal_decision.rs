use crate::ApprovalDecision;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FeatureProposalDecision {
    pub expected_revision: i64,
    pub decision: ApprovalDecision,
    #[serde(default)]
    pub reason: String,
}
