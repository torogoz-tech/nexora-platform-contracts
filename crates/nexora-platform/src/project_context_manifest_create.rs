use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectContextManifestCreate {
    pub name: String,
    pub repository_ids: Vec<String>,
    pub reason: String,
}
