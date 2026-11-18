use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GitlabWebhookRegistration {
    pub signing_token: String,
    pub expected_revision: i64,
}
