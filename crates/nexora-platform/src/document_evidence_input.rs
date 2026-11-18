use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentEvidenceInput {
    pub repository_id: String,
    pub path: String,
    pub start_line: usize,
    pub line_count: usize,
}
