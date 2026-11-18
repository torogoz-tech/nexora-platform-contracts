use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeveloperWorkflowAssignment {
    pub actor_id: String,
    pub actor_name: String,
    pub workflow_id: String,
    pub workflow_version_id: String,
    pub revision: i64,
    pub assigned_by: String,
    pub updated_at: String,
}
