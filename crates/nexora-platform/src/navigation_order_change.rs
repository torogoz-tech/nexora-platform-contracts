use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NavigationOrderChange {
    pub expected_revision: i64,
    pub paths: Vec<String>,
}
