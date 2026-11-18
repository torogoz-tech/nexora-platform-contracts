use crate::ArchitectureDeclaration;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectDocumentBody {
    pub text: String,
    pub source: String,
    #[serde(default)]
    pub architecture: Option<ArchitectureDeclaration>,
}
