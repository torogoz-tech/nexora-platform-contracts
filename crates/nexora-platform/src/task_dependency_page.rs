use crate::TaskDependency;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskDependencyPage {
    pub items: Vec<TaskDependency>,
    pub next_after: Option<String>,
}
