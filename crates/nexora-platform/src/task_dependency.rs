use crate::{TaskDependencyKind, TaskDependencyStatus};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskDependency {
    pub id: String,
    pub project_id: Option<String>,
    pub source_task_id: String,
    pub target_task_id: String,
    pub kind: TaskDependencyKind,
    pub status: TaskDependencyStatus,
    pub revision: i64,
    pub created_by: String,
    pub created_at: String,
    pub updated_at: String,
    pub resolved_at: Option<String>,
    pub resolved_by: Option<String>,
    pub resolution_reason: Option<String>,
    pub removed_at: Option<String>,
    pub removed_by: Option<String>,
    pub removal_reason: Option<String>,
}
