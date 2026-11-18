use crate::WorkflowGuard;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkflowTransition {
    pub id: String,
    pub from: String,
    pub to: String,
    #[serde(default)]
    pub name: String,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub automatic: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub steps: Vec<crate::WorkflowTransitionStep>,
    #[serde(default)]
    pub guards: Vec<WorkflowGuard>,
}

impl WorkflowTransition {
    pub fn requires_approval(&self) -> bool {
        self.guards.contains(&WorkflowGuard::HumanApproval)
            || self
                .steps
                .iter()
                .any(|step| matches!(step, crate::WorkflowTransitionStep::RequireApproval))
    }
}
