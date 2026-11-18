use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskType {
    #[serde(default)]
    pub purpose: crate::TaskPurpose,
    #[serde(default)]
    pub icon: crate::TaskTypeIcon,
    pub code: String,
    pub project_id: String,
    pub name: String,
    pub color: Option<crate::HexColor>,
    pub description: Option<String>,
    pub archived: bool,
    pub revision: i64,
}
