use crate::ProjectContextMember;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectContextManifest {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub members: Vec<ProjectContextMember>,
    pub checksum: String,
    pub reason: String,
    pub created_by: String,
    pub created_at: String,
}
