use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskWorkPrepare {
    pub transition_id: String,
    pub parameters: crate::TaskTransitionRequest,
    pub repository_ids: Vec<String>,
}
