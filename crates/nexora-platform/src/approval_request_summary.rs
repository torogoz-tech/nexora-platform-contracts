use crate::ApprovalRequestStatus;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApprovalRequestSummary {
    pub id: String,
    pub project_id: String,
    pub target_kind: String,
    pub resource_id: String,
    pub version_id: Option<String>,
    pub transition_id: String,
    pub workflow_version_id: String,
    pub status: ApprovalRequestStatus,
    pub revision: i64,
    pub requested_by: String,
    pub created_at: String,
    pub expires_at: String,
    pub approval_id: Option<String>,
}
