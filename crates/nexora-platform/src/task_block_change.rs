use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskBlockChange {
    pub blocked: bool,
    pub reason: Option<String>,
    pub blocker_id: Option<String>,
}
