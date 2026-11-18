use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskTransitionBranch {
    pub repository_id: String,
    pub branch_name: String,
    pub expected_head_sha: String,
}
