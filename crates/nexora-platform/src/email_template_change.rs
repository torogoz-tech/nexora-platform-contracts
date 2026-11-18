use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EmailTemplateChange {
    pub expected_revision: i64,
    pub html: String,
}
