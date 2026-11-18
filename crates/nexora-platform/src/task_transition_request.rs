use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields, default)]
pub struct TaskTransitionRequest {
    pub expected_revision: i64,
    pub reason: Option<String>,
    pub resolution: Option<String>,
    pub clear_resolution: bool,
    pub approval_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_revision: Option<i64>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub branches: Vec<crate::TaskTransitionBranch>,
}
