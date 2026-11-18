use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceCoverage {
    pub path: String,
    pub relation: String,
    pub domain: String,
    pub complete: bool,
}
