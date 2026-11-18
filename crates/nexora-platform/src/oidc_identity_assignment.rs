use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OidcIdentityAssignment {
    pub provider_id: String,
    pub subject: String,
}
