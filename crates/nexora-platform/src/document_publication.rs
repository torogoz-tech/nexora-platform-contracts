use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentPublication {
    pub id: String,
    pub project_id: String,
    pub document_id: String,
    pub version_id: String,
    pub checksum: String,
    pub workflow_version_id: Option<String>,
    pub state_id: String,
    pub state_revision: i64,
    pub published_by: String,
    pub reason: String,
    pub created_at: String,
}
