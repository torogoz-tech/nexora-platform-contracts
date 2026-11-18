use crate::{TaskDependencyKind, TaskDependencyStatus};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct TaskDependencyQuery {
    pub after: Option<String>,
    pub limit: u16,
    pub include_removed: bool,
    pub kind: Option<TaskDependencyKind>,
    pub status: Option<TaskDependencyStatus>,
}
impl Default for TaskDependencyQuery {
    fn default() -> Self {
        Self {
            after: None,
            limit: 50,
            include_removed: false,
            kind: None,
            status: None,
        }
    }
}
