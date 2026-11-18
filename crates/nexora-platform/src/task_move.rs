use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TaskMove {
    pub expected_revision: i64,
    pub backlog_id: String,
    pub expected_source_backlog_revision: Option<i64>,
    pub expected_target_backlog_revision: i64,
    pub reason: String,
}
