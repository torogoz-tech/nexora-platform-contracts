use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyntaxCapture {
    pub span: crate::SourceSpan,
    pub token_start: usize,
    pub token_end: usize,
}
