use serde::{Deserialize, Serialize};
/// An immutable completion of one internal workflow stage, independent of publication.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskStageCompletion {
    pub task_id: String,
    pub workflow_version_id: String,
    pub state_id: String,
    pub task_revision: i64,
    pub completed_at: String,
}
