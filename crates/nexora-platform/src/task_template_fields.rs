use crate::TaskPriority;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields, default)]
pub struct TaskTemplateFields {
    pub title: Option<String>,
    pub context: String,
    pub type_code: String,
    pub priority: TaskPriority,
    pub fields: serde_json::Map<String, serde_json::Value>,
}
