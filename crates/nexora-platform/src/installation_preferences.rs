use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallationPreferences {
    pub language: crate::InstallationLanguage,
    pub revision: i64,
}
