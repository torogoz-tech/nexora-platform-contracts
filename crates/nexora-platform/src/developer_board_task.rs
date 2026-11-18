use crate::TaskSummary;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeveloperBoardTask {
    #[serde(flatten)]
    pub task: TaskSummary,
    pub project_name: Option<String>,
}
