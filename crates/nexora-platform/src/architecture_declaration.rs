use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArchitectureDeclaration {
    pub style: String,
    pub layers: Vec<String>,
    pub conventions: Vec<String>,
    pub dependency_restrictions: Vec<String>,
}
