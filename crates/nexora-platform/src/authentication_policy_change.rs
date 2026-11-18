use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AuthenticationPolicyChange {
    pub local_enabled: bool,
    pub expected_revision: i64,
}
