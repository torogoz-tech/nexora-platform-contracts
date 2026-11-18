use crate::WorkflowGraph;
use crate::WorkflowIcon;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeveloperWorkflowCreate {
    pub name: String,
    #[serde(default, skip_serializing_if = "WorkflowIcon::is_default")]
    pub icon: WorkflowIcon,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(default)]
    pub purpose: String,
    pub graph: WorkflowGraph,
}
