use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskSubtaskCreate {
    pub expected_revision: i64,
    #[serde(default)]
    pub work_kind: crate::TaskSubtaskKind,
    #[serde(default)]
    pub tested_repository_id: Option<String>,
    #[serde(default)]
    pub tested_commit_sha: Option<String>,
    pub definition: crate::TaskCreate,
}
