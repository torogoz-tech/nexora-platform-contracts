use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskWorkReport {
    pub branches: Vec<crate::TaskWorkBranchReport>,
}
