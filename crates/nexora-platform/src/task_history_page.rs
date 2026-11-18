use crate::TaskHistoryEntry;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskHistoryPage {
    pub items: Vec<TaskHistoryEntry>,
    pub next_after: Option<i64>,
}
