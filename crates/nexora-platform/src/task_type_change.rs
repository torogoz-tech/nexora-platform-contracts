use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TaskTypeChange {
    pub expected_revision: i64,
    #[serde(default, skip_serializing_if = "crate::ColorChange::is_unchanged")]
    pub color: crate::ColorChange,
    pub name: Option<String>,
    pub description: Option<String>,
    pub archived: Option<bool>,
    pub reason: String,
}
