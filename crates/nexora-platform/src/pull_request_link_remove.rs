use serde::{Deserialize, Serialize};
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PullRequestLinkRemove {
    pub expected_revision: i64,
    pub reason: String,
}
