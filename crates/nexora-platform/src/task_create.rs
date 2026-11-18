use crate::TaskPriority;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields, default)]
pub struct TaskCreate {
    #[serde(default)]
    pub entry_workflow_id: Option<String>,
    #[serde(default)]
    pub primary_requirement_id: Option<String>,
    #[serde(default)]
    pub primary_feature_id: Option<String>,
    pub title: String,
    #[serde(default)]
    pub acceptance_criteria: Vec<String>,
    pub context: String,
    pub type_code: String,
    pub backlog_id: Option<String>,
    pub workflow_version_id: Option<String>,
    pub priority: TaskPriority,
    pub assignee_id: Option<String>,
    pub origin: String,
    pub origin_reference: Option<String>,
    pub fields: serde_json::Map<String, serde_json::Value>,
}
