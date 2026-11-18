use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BacklogReorder {
    pub expected_revision: i64,
    pub task_id: String,
    pub before_id: Option<String>,
    pub after_id: Option<String>,
}
