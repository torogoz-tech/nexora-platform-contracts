use crate::{DocumentLinkKind, DocumentLinkTarget};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentLinkCreate {
    pub expected_document_revision: i64,
    pub version_id: String,
    pub target: DocumentLinkTarget,
    pub kind: DocumentLinkKind,
    pub reason: String,
}
