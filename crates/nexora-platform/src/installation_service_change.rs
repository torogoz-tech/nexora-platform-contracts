use serde::{Deserialize, Serialize};
use zeroize::Zeroize;
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InstallationServiceChange {
    pub settings: serde_json::Value,
    #[serde(skip_serializing)]
    pub secret: Option<String>,
    #[serde(default)]
    pub remove_secret: bool,
    pub expected_revision: i64,
}
impl Drop for InstallationServiceChange {
    fn drop(&mut self) {
        if let Some(secret) = &mut self.secret {
            secret.zeroize();
        }
    }
}
