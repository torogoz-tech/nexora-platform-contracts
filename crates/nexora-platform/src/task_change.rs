use crate::{TaskBlockChange, TaskPriority};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields, default)]
pub struct TaskChange {
    pub entry_workflow_id: Option<String>,
    pub clear_entry_workflow: bool,
    pub primary_requirement_id: Option<String>,
    pub primary_feature_id: Option<String>,
    pub clear_primary_knowledge: bool,
    pub expected_revision: i64,
    pub title: Option<String>,
    #[serde(default)]
    pub acceptance_criteria: Option<Vec<String>>,
    pub context: Option<String>,
    pub type_code: Option<String>,
    pub priority: Option<TaskPriority>,
    pub assignee_id: Option<String>,
    pub clear_assignee: bool,
    pub assignment_state_id: Option<String>,
    pub assignment_workflow_version_id: Option<String>,
    pub block: Option<TaskBlockChange>,
    pub fields: Option<serde_json::Map<String, serde_json::Value>>,
    pub archived: Option<bool>,
    pub reason: String,
}
