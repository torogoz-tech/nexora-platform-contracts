use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextPackageExport {
    pub package_id: String,
    pub checksum: String,
    pub media_type: String,
    pub content: String,
}
