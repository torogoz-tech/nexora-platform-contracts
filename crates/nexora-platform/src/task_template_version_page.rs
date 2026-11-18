use crate::TaskTemplateVersionSummary;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskTemplateVersionPage {
    pub items: Vec<TaskTemplateVersionSummary>,
    pub next_before: Option<i64>,
}
