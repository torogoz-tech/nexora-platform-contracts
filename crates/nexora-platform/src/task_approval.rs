use crate::TaskTransitionRequest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskApproval {
    pub id: String,
    pub project_id: Option<String>,
    pub task_id: String,
    pub task_revision: i64,
    pub workflow_id: String,
    pub workflow_version_id: String,
    pub transition_id: String,
    pub from_state: String,
    pub to_state: String,
    pub parameters: TaskTransitionRequest,
    pub parameters_checksum: String,
    pub reason: String,
    pub granted_by: String,
    pub credential_id: String,
    pub created_at: String,
    pub expires_at: String,
    pub consumed_at: Option<String>,
    pub consumed_by: Option<String>,
    pub consumed_credential_id: Option<String>,
    pub revoked_at: Option<String>,
    pub revoked_by: Option<String>,
    pub revocation_reason: Option<String>,
}
