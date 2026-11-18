use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InstallationPreferencesChange {
    pub language: crate::InstallationLanguage,
    pub expected_revision: i64,
}
