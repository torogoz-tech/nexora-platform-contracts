use serde::{Deserialize, Serialize};
/// Complete replacement of editable project fields, guarded by the observed revision.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectChange {
    pub expected_revision: i64,
    #[serde(default, skip_serializing_if = "crate::ColorChange::is_unchanged")]
    pub color: crate::ColorChange,
    pub name: String,
    pub description: Option<String>,
    pub archived: bool,
}
