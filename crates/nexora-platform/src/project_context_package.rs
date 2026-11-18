use crate::ProjectContextPackageContent;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectContextPackage {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub content: ProjectContextPackageContent,
    pub checksum: String,
    pub reason: String,
    pub created_by: String,
    pub created_at: String,
    #[serde(default)]
    pub needs_review: bool,
}
