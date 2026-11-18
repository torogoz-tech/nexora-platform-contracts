use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskTake {
    pub expected_revision: i64,
    #[serde(default)]
    pub workflow_id: Option<String>,
}
