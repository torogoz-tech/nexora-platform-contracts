use crate::WorkflowResourceKind;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentSummary {
    pub id: String,
    pub project_id: String,
    pub code: String,
    pub kind: WorkflowResourceKind,
    pub archived: bool,
    pub revision: i64,
    pub latest_version_id: String,
    pub latest_version_number: i64,
    pub published_version_id: Option<String>,
    pub created_by: String,
    pub created_at: String,
    pub updated_at: String,
    pub latest_title: String,
    pub latest_checksum: String,
    pub workflow_version_id: Option<String>,
    pub state_id: String,
    pub state_revision: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context_manifest_id: Option<String>,
    #[serde(default)]
    pub needs_review: bool,
}
