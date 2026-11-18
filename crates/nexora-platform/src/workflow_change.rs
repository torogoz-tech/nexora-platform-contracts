use crate::WorkflowIcon;
use crate::{WorkflowGraph, WorkflowResourceKind};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkflowChange {
    pub expected_revision: i64,
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<WorkflowIcon>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    pub purpose: Option<String>,
    pub resource_kinds: Option<Vec<WorkflowResourceKind>>,
    pub graph: Option<WorkflowGraph>,
    pub archived: Option<bool>,
    pub reason: String,
}
