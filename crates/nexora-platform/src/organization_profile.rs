use serde::{Deserialize, Serialize};
#[derive(Clone, Deserialize, Serialize)]
pub struct OrganizationProfile {
    pub name: String,
    pub address: String,
    pub phone: String,
    pub country_code: String,
    pub has_logo: bool,
    pub revision: i64,
}
