use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContextDocumentReference {
    pub document_id: String,
    pub version_id: String,
}
