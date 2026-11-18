use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DevelopmentPolicyChange {
    pub expected_revision: i64,
    pub base_branch: String,
    pub branch_prefix: String,
}
