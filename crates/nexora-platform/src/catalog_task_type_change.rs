use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CatalogTaskTypeChange {
    pub expected_revision: i64,
    pub name: Option<String>,
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "crate::ColorChange::is_unchanged")]
    pub color: crate::ColorChange,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<crate::TaskTypeIcon>,
    pub archived: Option<bool>,
}
