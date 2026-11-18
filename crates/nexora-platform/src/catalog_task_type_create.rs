use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CatalogTaskTypeCreate {
    #[serde(default, skip_serializing_if = "crate::TaskPurpose::is_general")]
    pub purpose: crate::TaskPurpose,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<crate::HexColor>,
    #[serde(default, skip_serializing_if = "crate::TaskTypeIcon::is_default")]
    pub icon: crate::TaskTypeIcon,
}
