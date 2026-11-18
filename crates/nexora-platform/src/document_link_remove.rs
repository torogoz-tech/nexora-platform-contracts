use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentLinkRemove {
    pub expected_document_revision: i64,
    pub expected_target_revision: i64,
    pub reason: String,
}
