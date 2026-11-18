use crate::TaskComment;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskCommentPage {
    pub items: Vec<TaskComment>,
    pub next_after: Option<String>,
}
