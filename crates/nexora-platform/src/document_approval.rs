use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DocumentApproval {
    pub id: String,
    pub project_id: String,
    pub document_id: String,
    pub version_id: String,
    pub document_revision: i64,
    pub state_revision: i64,
    pub content_checksum: String,
    pub workflow_version_id: String,
    pub transition_id: String,
    pub from_state: String,
    pub to_state: String,
    pub parameters: Value,
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
