use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaskWorkSession {
    pub id: String,
    pub project_id: String,
    pub task_id: String,
    pub actor_id: String,
    pub task_revision: i64,
    pub workflow_version_id: String,
    pub transition_id: String,
    pub parameters: crate::TaskTransitionRequest,
    pub branches: Vec<crate::TaskWorkBranch>,
    pub report: Option<crate::TaskWorkReportReceipt>,
}
