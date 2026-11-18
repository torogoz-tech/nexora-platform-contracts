use crate::TaskPriority;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct TaskListQuery {
    pub backlog_id: Option<String>,
    pub assignee_id: Option<String>,
    pub type_code: Option<String>,
    pub priority: Option<TaskPriority>,
    pub state_id: Option<String>,
    pub workflow_version_id: Option<String>,
    pub after: Option<String>,
    pub limit: u16,
    pub include_archived: bool,
}
impl Default for TaskListQuery {
    fn default() -> Self {
        Self {
            backlog_id: None,
            assignee_id: None,
            type_code: None,
            priority: None,
            state_id: None,
            workflow_version_id: None,
            after: None,
            limit: 50,
            include_archived: false,
        }
    }
}
