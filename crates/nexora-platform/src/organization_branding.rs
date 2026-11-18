use serde::{Deserialize, Serialize};
#[derive(Clone, Deserialize, Serialize)]
pub struct OrganizationBranding {
    pub name: String,
    pub has_logo: bool,
    pub revision: i64,
}
