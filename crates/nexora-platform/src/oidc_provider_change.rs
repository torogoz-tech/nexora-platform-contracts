use serde::{Deserialize, Serialize};
use zeroize::Zeroize;
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OidcProviderChange {
    pub expected_revision: i64,
    pub name: Option<String>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub redirect_uri: Option<String>,
    pub enabled: Option<bool>,
}
impl Drop for OidcProviderChange {
    fn drop(&mut self) {
        if let Some(secret) = self.client_secret.as_mut() {
            secret.zeroize();
        }
    }
}
