use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskTemplate {
    pub id: String,
    pub project_id: String,
    pub code: String,
    pub name: String,
    pub archived: bool,
    pub revision: i64,
    pub latest_version_id: Option<String>,
    pub latest_version_number: i64,
    pub created_by: String,
    pub created_at: String,
    pub updated_at: String,
}
