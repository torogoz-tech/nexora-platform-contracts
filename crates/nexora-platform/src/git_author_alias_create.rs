use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GitAuthorAliasCreate {
    pub provider: String,
    pub host: String,
    pub email: String,
    /// Accepted for older clients; aliases always apply to the whole organization.
    #[serde(default)]
    pub project_id: Option<String>,
    #[serde(default)]
    pub reason: String,
}
