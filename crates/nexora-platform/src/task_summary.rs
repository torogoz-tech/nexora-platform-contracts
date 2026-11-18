use crate::TaskPriority;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskSummary {
    pub id: String,
    #[serde(default)]
    pub lifecycle: crate::TaskLifecycle,
    #[serde(default)]
    pub purpose: crate::TaskPurpose,
    #[serde(default)]
    pub entry_workflow_id: Option<String>,
    pub project_id: Option<String>,
    pub title: String,
    pub type_code: String,
    pub priority: TaskPriority,
    pub assignee_id: Option<String>,
    pub backlog_id: Option<String>,
    pub workflow_version_id: Option<String>,
    pub state_id: Option<String>,
    pub blocked: bool,
    #[serde(default)]
    pub blocking_count: i64,
    #[serde(default)]
    pub subtasks_total: i64,
    #[serde(default)]
    pub subtasks_completed: i64,
    pub archived: bool,
    pub position: Option<i64>,
    pub revision: i64,
}
