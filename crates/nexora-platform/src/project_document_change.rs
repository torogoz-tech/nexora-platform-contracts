use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectDocumentChange {
    pub expected_revision: i64,
    pub archived: bool,
    pub reason: String,
}
