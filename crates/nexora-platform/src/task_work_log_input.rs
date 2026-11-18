use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskWorkLogInput {
    pub date: String,
    pub overtime_request_id: Option<String>,
    pub minutes: i32,
    pub note: Option<String>,
    pub expected_revision: Option<i64>,
}
