use crate::Actor;
use serde::Serialize;
#[derive(Debug, Serialize)]
pub struct BootstrapResult {
    pub installation_id: String,
    pub organization_id: String,
    pub organization_name: String,
    pub owner: Actor,
}
