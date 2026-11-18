use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SyntaxCoverage {
    pub hierarchical: bool,
    pub parsed_regions: usize,
    pub partial_regions: usize,
    pub opaque_regions: usize,
}
