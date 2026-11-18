use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisibleTaskType {
    #[serde(default)]
    pub icon: crate::TaskTypeIcon,
    pub code: String,
    pub name: String,
    pub color: Option<String>,
    pub total: i64,
}
