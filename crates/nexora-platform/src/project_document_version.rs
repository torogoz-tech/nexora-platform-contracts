use crate::DocumentOrigin;
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDocumentVersion {
    pub id: String,
    pub project_id: String,
    pub document_id: String,
    pub number: i64,
    pub previous_version_id: Option<String>,
    pub title: String,
    pub content: Value,
    pub origin: DocumentOrigin,
    pub workflow_version_id: Option<String>,
    pub state_id: String,
    pub state_revision: i64,
    pub checksum: String,
    pub reason: String,
    pub created_by: String,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context_manifest_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence: Vec<crate::DocumentEvidence>,
    #[serde(default)]
    pub needs_review: bool,
}
