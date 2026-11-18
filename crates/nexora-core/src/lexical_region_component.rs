use crate::{LexicalRegion, SourceSpan};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LexicalRegionComponent {
    #[default]
    Content,
    Header,
}
impl LexicalRegionComponent {
    pub fn name(self) -> &'static str {
        match self {
            Self::Content => "content",
            Self::Header => "header",
        }
    }
    pub fn capture(self) -> &'static str {
        match self {
            Self::Content => "embedded_content",
            Self::Header => "embedded_header",
        }
    }
    pub fn from_capture(name: &str) -> Option<Self> {
        match name {
            "embedded_content" => Some(Self::Content),
            "embedded_header" => Some(Self::Header),
            _ => None,
        }
    }
    pub fn input(self, region: &LexicalRegion) -> Option<SourceSpan> {
        match self {
            Self::Content => Some(region.content_span),
            Self::Header => region.header_content_span,
        }
    }
    pub fn coverage(self, region: &LexicalRegion) -> Option<SourceSpan> {
        match self {
            Self::Content => Some(region.content_span),
            Self::Header => region.header,
        }
    }
    pub fn protected(self, region: &LexicalRegion) -> Option<SourceSpan> {
        match self {
            Self::Content => Some(region.span),
            Self::Header => region.header,
        }
    }
    pub fn closed(self, region: &LexicalRegion) -> bool {
        match self {
            Self::Content => region.closed,
            Self::Header => region.header_closed,
        }
    }
}
