use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogTaskType {
    #[serde(default)]
    pub purpose: crate::TaskPurpose,
    #[serde(default)]
    pub icon: crate::TaskTypeIcon,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub color: Option<crate::HexColor>,
    pub archived: bool,
    pub revision: i64,
}
