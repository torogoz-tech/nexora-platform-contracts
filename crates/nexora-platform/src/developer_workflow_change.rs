use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeveloperWorkflowChange {
    pub expected_revision: i64,
    pub workflow_version_id: String,
    #[serde(default)]
    pub reason: String,
}
