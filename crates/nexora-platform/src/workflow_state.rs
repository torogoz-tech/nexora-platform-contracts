use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkflowState {
    pub id: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_base_branch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_order: Option<i32>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub prevent_backtracking: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<crate::HexColor>,
    #[serde(default)]
    pub initial: bool,
    #[serde(default)]
    pub terminal: bool,
    #[serde(default, skip_serializing_if = "WorkflowState::not_publishable")]
    pub publishable: bool,
}

impl WorkflowState {
    fn not_publishable(value: &bool) -> bool {
        !value
    }
}
