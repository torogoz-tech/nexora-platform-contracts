use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowVersionSummary {
    pub id: String,
    pub workflow_id: String,
    pub project_id: Option<String>,
    pub number: i64,
    pub name: String,
    pub purpose: String,
    pub checksum: String,
    pub reason: String,
    pub published_by: String,
    pub published_at: String,
}
