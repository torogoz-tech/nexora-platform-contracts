use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GitConfigurationStateChange {
    pub enabled: bool,
    pub expected_revision: i64,
}
