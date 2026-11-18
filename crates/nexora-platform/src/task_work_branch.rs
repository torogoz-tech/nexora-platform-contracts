use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaskWorkBranch {
    pub repository_id: String,
    pub base_branch: String,
    pub branch_prefix: String,
    pub branch_name: String,
    pub policy_revision: i64,
    pub source_authority: serde_json::Value,
    pub base_sha: Option<String>,
    pub head_sha: Option<String>,
}
