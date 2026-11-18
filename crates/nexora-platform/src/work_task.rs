use crate::TaskPriority;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkTask {
    #[serde(default)]
    pub governed_stage: bool,
    #[serde(default)]
    pub purpose: crate::TaskPurpose,
    #[serde(default)]
    pub entry_workflow_id: Option<String>,
    #[serde(default)]
    pub primary_requirement_id: Option<String>,
    #[serde(default)]
    pub primary_feature_id: Option<String>,
    pub id: String,
    #[serde(default)]
    pub lifecycle: crate::TaskLifecycle,
    pub project_id: Option<String>,
    pub title: String,
    #[serde(default)]
    pub acceptance_criteria: Vec<String>,
    pub context: String,
    pub type_code: String,
    pub priority: TaskPriority,
    pub assignee_id: Option<String>,
    pub backlog_id: Option<String>,
    pub workflow_id: Option<String>,
    pub workflow_version_id: Option<String>,
    pub state_id: Option<String>,
    pub origin: String,
    pub origin_reference: Option<String>,
    pub resolution: Option<String>,
    pub blocked: bool,
    pub block_reason: Option<String>,
    pub blocker_id: Option<String>,
    pub fields: serde_json::Map<String, serde_json::Value>,
    pub position: Option<i64>,
    pub archived: bool,
    pub revision: i64,
    pub template_version_id: Option<String>,
    pub template_snapshot: Option<serde_json::Value>,
    pub created_by: String,
    pub created_at: String,
    pub updated_at: String,
}
impl WorkTask {
    pub fn execution_version(&self) -> Result<&str, crate::PlatformError> {
        self.workflow_version_id
            .as_deref()
            .ok_or(crate::PlatformError::Conflict("task has not been taken"))
    }
    pub fn execution_state(&self) -> Result<&str, crate::PlatformError> {
        self.state_id
            .as_deref()
            .ok_or(crate::PlatformError::Conflict("task has not been taken"))
    }
    pub fn project_scope(&self) -> Result<&str, crate::PlatformError> {
        self.project_id
            .as_deref()
            .ok_or(crate::PlatformError::Validation(
                "this operation requires a project task",
            ))
    }
}
