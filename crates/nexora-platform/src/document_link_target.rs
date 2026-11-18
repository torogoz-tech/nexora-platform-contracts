use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum DocumentLinkTarget {
    Task {
        task_id: String,
        expected_revision: i64,
    },
    DocumentVersion {
        document_id: String,
        version_id: String,
        expected_revision: i64,
    },
}
