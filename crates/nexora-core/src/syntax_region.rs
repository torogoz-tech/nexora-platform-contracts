use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyntaxRegion {
    pub capture: String,
    pub span: crate::SourceSpan,
    pub token_start: usize,
    pub token_end: usize,
    pub context: Option<String>,
    pub status: crate::RegionStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coverage: Option<crate::RegionCoverageProof>,
}
