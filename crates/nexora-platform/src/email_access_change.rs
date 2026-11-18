use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EmailAccessChange {
    pub public_web_url: String,
    pub expected_revision: i64,
}
