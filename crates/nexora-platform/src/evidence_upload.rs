use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceUpload {
    pub repository_id: String,
    pub snapshot: Option<String>,
    pub document_id: Option<String>,
    pub document_version_id: Option<String>,
    pub name: String,
    pub media_type: String,
    pub data_base64: String,
    pub phase: String,
    pub description: String,
    #[serde(default)]
    pub provided_metadata: Value,
}
