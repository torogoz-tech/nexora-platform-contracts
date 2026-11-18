use crate::ContextDocumentReference;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectContextPackageCreate {
    pub name: String,
    #[serde(default)]
    pub context_manifest_id: Option<String>,
    #[serde(default)]
    pub document_versions: Vec<ContextDocumentReference>,
    #[serde(default)]
    pub task_ids: Vec<String>,
    pub reason: String,
}
