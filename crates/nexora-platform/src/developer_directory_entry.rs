use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeveloperDirectoryEntry {
    pub id: String,
    pub name: String,
    pub workflow_id: Option<String>,
    pub workflow_version_id: Option<String>,
    pub assignment_revision: i64,
    pub role: Option<crate::MemberRole>,
    pub workflows: Vec<crate::DeveloperWorkflowAssignment>,
}
