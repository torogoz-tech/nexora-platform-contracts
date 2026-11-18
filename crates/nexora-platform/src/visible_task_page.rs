use crate::{TaskParticipant, VisibleTask};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisibleTaskPage {
    pub items: Vec<VisibleTask>,
    pub next_after: Option<String>,
    pub total: i64,
    pub participants: Vec<TaskParticipant>,
    /// Whether this credential can include organization tasks without a project.
    #[serde(default)]
    pub projectless_tasks_included: bool,
}
