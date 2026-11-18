use crate::{FeatureLinkProposal, FeatureTaskProposal};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FeatureProposalCreate {
    pub requirement_id: String,
    pub requirement_version_id: String,
    pub expected_catalog_revision: i64,
    pub reason: String,
    pub links: Vec<FeatureLinkProposal>,
    pub tasks: Vec<FeatureTaskProposal>,
}
