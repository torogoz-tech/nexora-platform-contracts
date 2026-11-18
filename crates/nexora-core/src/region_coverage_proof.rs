use crate::{NodeId, RegionCoverageMethod};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionCoverageProof {
    pub method: RegionCoverageMethod,
    pub regions: Vec<NodeId>,
    pub syntax_units: Vec<NodeId>,
}
