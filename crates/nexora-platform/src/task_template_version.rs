use crate::TaskTemplateFields;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskTemplateVersion {
    pub id: String,
    pub project_id: String,
    pub template_id: String,
    pub number: i64,
    pub previous_version_id: Option<String>,
    pub code: String,
    pub name: String,
    pub definition: TaskTemplateFields,
    pub valid_from: String,
    pub valid_until: Option<String>,
    pub checksum: String,
    pub reason: String,
    pub published_by: String,
    pub created_at: String,
}
