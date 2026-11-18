use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectDocumentPublish {
    pub expected_revision: i64,
    pub version_id: String,
    #[serde(default = "ProjectDocumentPublish::initial_state_revision")]
    pub expected_state_revision: i64,
    pub reason: String,
}
impl ProjectDocumentPublish {
    fn initial_state_revision() -> i64 {
        1
    }
}
