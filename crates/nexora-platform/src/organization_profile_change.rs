use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OrganizationProfileChange {
    pub expected_revision: i64,
    pub name: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub country_code: Option<String>,
}
