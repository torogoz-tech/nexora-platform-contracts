use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Backlog {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub color: Option<crate::HexColor>,
    pub description: Option<String>,
    pub workflow_id: String,
    pub workflow_version_id: String,
    pub archived: bool,
    pub revision: i64,
    pub created_by: String,
    pub created_at: String,
    pub updated_at: String,
}
