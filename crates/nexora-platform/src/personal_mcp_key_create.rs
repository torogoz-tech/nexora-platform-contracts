use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PersonalMcpKeyCreate {
    pub name: String,
    /// Legacy client hint, validated but not persisted as a project restriction.
    #[serde(default)]
    pub project_ids: Vec<String>,
    #[serde(default)]
    pub read_only: bool,
    /// Legacy field; personal keys inherit live global grants automatically.
    #[serde(default)]
    pub include_global: bool,
    /// `0` creates a key without an expiration date.
    pub expires_in_seconds: i64,
}
