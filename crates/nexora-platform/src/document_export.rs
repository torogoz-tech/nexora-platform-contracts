use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentExport {
    pub document_id: String,
    pub version_id: String,
    pub checksum: String,
    pub media_type: String,
    pub content: String,
}
