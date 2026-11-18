use serde::{Deserialize, Serialize};
#[derive(Clone, Deserialize, Serialize)]
pub struct InstallationServiceSettings {
    pub settings: serde_json::Value,
    pub has_secret: bool,
    pub managed: bool,
    pub revision: i64,
}
