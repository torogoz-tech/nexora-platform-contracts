use serde::{Deserialize, Serialize};
/// Public Take contract; repository selection is explicit and never inferred from all project repositories.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskCodeTake {
    pub expected_revision: i64,
    #[serde(default)]
    pub workflow_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repository_ids: Vec<String>,
}
impl TaskCodeTake {
    pub fn take(&self) -> crate::TaskTake {
        crate::TaskTake {
            expected_revision: self.expected_revision,
            workflow_id: self.workflow_id.clone(),
        }
    }
}
