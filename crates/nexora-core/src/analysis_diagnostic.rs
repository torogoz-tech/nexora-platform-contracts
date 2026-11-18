use crate::SourceSpan;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisDiagnostic {
    pub stage: String,
    pub path: Option<String>,
    pub message: String,
    pub span: Option<SourceSpan>,
}
