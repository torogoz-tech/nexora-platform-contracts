use crate::WorkflowVersionSummary;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowVersionPage {
    pub items: Vec<WorkflowVersionSummary>,
    pub next_before: Option<i64>,
}
