use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FeatureLinkProposal {
    pub feature_id: Option<String>,
    pub expected_revision: Option<i64>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub impact: String,
}
