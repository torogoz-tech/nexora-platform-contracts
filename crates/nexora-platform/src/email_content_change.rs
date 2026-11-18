use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EmailContentChange {
    pub expected_revision: i64,
    pub subject: String,
    pub body_html: String,
}
