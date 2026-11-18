use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields, default)]
pub struct TaskTemplateChange {
    pub expected_revision: i64,
    pub name: Option<String>,
    pub archived: Option<bool>,
    pub reason: String,
}
