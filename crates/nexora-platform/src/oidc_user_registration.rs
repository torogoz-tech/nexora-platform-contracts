use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OidcUserRegistration {
    pub username: String,
    pub name: String,
    pub provider_id: String,
    pub subject: String,
}
