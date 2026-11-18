use serde::{Deserialize, Serialize};
/// Client-reported Git metadata, never independent verification.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskWorkBranchReport {
    pub repository_id: String,
    pub branch_name: String,
    pub base_sha: String,
    pub head_sha: String,
}
impl TaskWorkBranchReport {
    pub fn valid_sha(value: &str) -> bool {
        matches!(value.len(), 40 | 64) && value.bytes().all(|c| c.is_ascii_hexdigit())
    }
}
