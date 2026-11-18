use crate::ProjectDocumentVersion;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageDocumentSnapshot {
    pub document_revision: i64,
    pub published_version_id: Option<String>,
    pub version: ProjectDocumentVersion,
}
