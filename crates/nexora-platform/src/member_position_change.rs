use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MemberPositionChange {
    pub position: String,
    pub expected_revision: i64,
}
