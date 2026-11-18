use crate::WorkflowResourceKind;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDocument {
    pub id: String,
    pub project_id: String,
    pub code: String,
    pub kind: WorkflowResourceKind,
    pub archived: bool,
    pub revision: i64,
    pub latest_version_id: String,
    pub latest_version_number: i64,
    pub published_version_id: Option<String>,
    pub created_by: String,
    pub created_at: String,
    pub updated_at: String,
}
