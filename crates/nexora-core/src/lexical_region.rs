use crate::{NodeId, SourceSpan};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LexicalRegion {
    pub id: NodeId,
    pub mode: String,
    pub ast_kind: String,
    pub parent: Option<NodeId>,
    pub span: SourceSpan,
    pub content_span: SourceSpan,
    pub opener: SourceSpan,
    pub closer: Option<SourceSpan>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lookahead_span: Option<SourceSpan>,
    pub closed: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub header: Option<SourceSpan>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub header_content_span: Option<SourceSpan>,
    #[serde(default)]
    pub header_closed: bool,
    #[serde(default)]
    pub self_closed: bool,
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub captures: std::collections::BTreeMap<String, SourceSpan>,
}
