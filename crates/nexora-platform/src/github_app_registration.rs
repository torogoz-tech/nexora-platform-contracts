use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GithubAppRegistration {
    pub app_id: String,
    pub client_id: String,
    pub client_secret: String,
    pub private_key_pem: String,
    pub webhook_secret: String,
    pub redirect_uri: String,
    pub enabled: bool,
    pub expected_revision: i64,
}
