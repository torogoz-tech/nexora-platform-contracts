use crate::Capability;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CapabilityGrant {
    pub capability: Capability,
    pub project_id: Option<String>,
}
impl CapabilityGrant {
    pub fn new(capability: Capability, project_id: Option<String>) -> Self {
        Self {
            capability,
            project_id,
        }
    }
}
