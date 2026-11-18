use crate::WorkflowSummary;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowPage {
    pub items: Vec<WorkflowSummary>,
    pub next_after: Option<String>,
}
