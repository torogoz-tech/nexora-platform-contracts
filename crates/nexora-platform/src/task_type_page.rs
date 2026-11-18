use crate::TaskType;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaskTypePage {
    pub items: Vec<TaskType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_after: Option<String>,
}
