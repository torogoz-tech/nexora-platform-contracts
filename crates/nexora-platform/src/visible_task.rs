use crate::TaskSummary;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisibleTask {
    #[serde(default)]
    pub type_icon: crate::TaskTypeIcon,
    #[serde(flatten)]
    pub task: TaskSummary,
    pub project_name: Option<String>,
    pub type_name: String,
    pub state_name: Option<String>,
    #[serde(default)]
    pub state_color: Option<crate::HexColor>,
    pub workflow_id: Option<String>,
    pub assignee_name: Option<String>,
    pub created_by: String,
    #[serde(default)]
    pub creator: Option<crate::MemberIdentitySummary>,
    #[serde(default)]
    pub assignee: Option<crate::MemberIdentitySummary>,
    #[serde(default)]
    pub created_by_name: Option<String>,
    pub updated_at: String,
    #[serde(default)]
    pub state_changed_at: Option<String>,
}
