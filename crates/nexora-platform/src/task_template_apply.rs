use crate::TaskPriority;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields, default)]
pub struct TaskTemplateApply {
    pub template_version_id: String,
    pub title: Option<String>,
    pub context: Option<String>,
    pub type_code: Option<String>,
    pub priority: Option<TaskPriority>,
    pub fields: Option<serde_json::Map<String, serde_json::Value>>,
    pub backlog_id: Option<String>,
    pub workflow_version_id: Option<String>,
    pub assignee_id: Option<String>,
    pub origin: String,
    pub origin_reference: Option<String>,
}
