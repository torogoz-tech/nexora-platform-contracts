use crate::TaskCreate;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FeatureTaskProposal {
    pub feature_index: usize,
    pub definition: TaskCreate,
}
