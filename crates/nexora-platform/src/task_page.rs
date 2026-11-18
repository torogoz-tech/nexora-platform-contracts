use crate::TaskSummary;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskPage {
    pub items: Vec<TaskSummary>,
    pub next_after: Option<String>,
}
