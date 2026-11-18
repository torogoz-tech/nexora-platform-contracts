use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectTaskTypeSelectionChange {
    pub codes: Vec<String>,
    pub expected_revision: i64,
}
