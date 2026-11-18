use crate::DocumentLinkKind;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentLink {
    pub id: String,
    pub project_id: String,
    pub source_document_id: String,
    pub source_version_id: String,
    pub kind: DocumentLinkKind,
    pub target_task_id: Option<String>,
    pub target_document_id: Option<String>,
    pub target_version_id: Option<String>,
    pub source_revision_at_creation: i64,
    pub target_revision_at_creation: i64,
    pub reason: String,
    pub created_by: String,
    pub credential_id: String,
    pub created_at: String,
    pub removed_at: Option<String>,
    pub removed_by: Option<String>,
    pub removal_reason: Option<String>,
}
