use crate::{Capability, PlatformError};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KeyScope {
    pub capabilities: Vec<Capability>,
    #[serde(default)]
    pub project_ids: Vec<String>,
    #[serde(default)]
    pub all_projects: bool,
    #[serde(default)]
    pub installation: bool,
}
impl KeyScope {
    pub fn validate(&self) -> Result<(), PlatformError> {
        if self.capabilities.is_empty()
            || self.capabilities.len() > 128
            || self.project_ids.len() > 256
            || (self.all_projects && !self.project_ids.is_empty())
            || (!self.installation && !self.all_projects && self.project_ids.is_empty())
            || self.capabilities.iter().collect::<BTreeSet<_>>().len() != self.capabilities.len()
            || self.project_ids.iter().collect::<BTreeSet<_>>().len() != self.project_ids.len()
            || self
                .project_ids
                .iter()
                .any(|id| id.is_empty() || id.len() > 128 || id.chars().any(char::is_control))
        {
            return Err(PlatformError::Validation("invalid key scope"));
        }
        Ok(())
    }
    pub fn allows(&self, capability: Capability, project: Option<&str>) -> bool {
        self.capabilities.contains(&capability)
            && match project {
                Some(id) => self.all_projects || self.project_ids.iter().any(|p| p == id),
                None => self.installation,
            }
    }
}
