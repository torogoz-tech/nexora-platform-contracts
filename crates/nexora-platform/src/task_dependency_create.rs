use crate::TaskDependencyKind;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskDependencyCreate {
    pub source_task_id: String,
    pub target_task_id: String,
    pub kind: TaskDependencyKind,
    pub source_revision: i64,
    pub target_revision: i64,
    pub reason: String,
}
