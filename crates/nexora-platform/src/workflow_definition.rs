use crate::WorkflowIcon;
use crate::{WorkflowGraph, WorkflowResourceKind};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowDefinition {
    pub id: String,
    pub project_id: Option<String>,
    pub name: String,
    #[serde(default)]
    pub icon: WorkflowIcon,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    pub purpose: String,
    pub resource_kinds: Vec<WorkflowResourceKind>,
    pub draft: WorkflowGraph,
    pub archived: bool,
    pub revision: i64,
    pub latest_version_id: Option<String>,
    pub latest_version_number: i64,
    pub created_by: String,
    pub created_at: String,
    pub updated_at: String,
}
