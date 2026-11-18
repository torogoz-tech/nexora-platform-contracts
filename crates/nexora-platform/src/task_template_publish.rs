use crate::TaskTemplateFields;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskTemplatePublish {
    pub expected_revision: i64,
    pub definition: TaskTemplateFields,
    pub valid_from: Option<String>,
    pub valid_until: Option<String>,
    pub reason: String,
}
