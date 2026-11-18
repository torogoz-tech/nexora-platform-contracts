use crate::DocumentOrigin;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentVersionSummary {
    pub id: String,
    pub number: i64,
    pub title: String,
    pub origin: DocumentOrigin,
    pub workflow_version_id: Option<String>,
    pub state_id: String,
    pub state_revision: i64,
    pub checksum: String,
    pub created_by: String,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context_manifest_id: Option<String>,
    #[serde(default)]
    pub needs_review: bool,
}
