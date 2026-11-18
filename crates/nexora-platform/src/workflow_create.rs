use crate::WorkflowIcon;
use crate::{WorkflowGraph, WorkflowResourceKind};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkflowCreate {
    pub name: String,
    #[serde(default, skip_serializing_if = "WorkflowIcon::is_default")]
    pub icon: WorkflowIcon,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(default)]
    pub purpose: String,
    pub resource_kinds: Vec<WorkflowResourceKind>,
    pub graph: WorkflowGraph,
}
