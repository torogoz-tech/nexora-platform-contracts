use crate::{DocumentContent, DocumentOrigin};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectDocumentRevise {
    pub expected_revision: i64,
    pub title: String,
    pub content: DocumentContent,
    pub origin: DocumentOrigin,
    pub reason: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context_manifest_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence: Vec<crate::DocumentEvidenceInput>,
}
