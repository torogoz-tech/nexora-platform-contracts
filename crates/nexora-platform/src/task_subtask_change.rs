use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskSubtaskChange {
    pub expected_revision: i64,
    pub child_revision: i64,
    pub reason: String,
}
