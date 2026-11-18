use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EvidenceRecord {
    pub id: String,
    pub project_id: String,
    pub repository_id: String,
    pub snapshot: Option<String>,
    pub document_id: Option<String>,
    pub document_version_id: Option<String>,
    pub name: String,
    pub media_type: String,
    pub phase: String,
    pub description: String,
    pub sha256: String,
    pub size_bytes: i64,
    pub provided_metadata: Value,
    pub created_by: String,
    pub created_at: String,
    pub trust: String,
}
