use crate::{MemberRole, PlatformError};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MemberRoleChange {
    pub role: MemberRole,
    pub all_projects: bool,
    pub project_ids: Vec<String>,
    pub expected_revision: i64,
}
impl MemberRoleChange {
    pub fn validate(&self) -> Result<(), PlatformError> {
        if self.role == MemberRole::SuperAdmin
            || self.expected_revision < 0
            || self.project_ids.len() > 1000
            || (self.all_projects && !self.project_ids.is_empty())
            || (matches!(self.role, MemberRole::User | MemberRole::Supervisor)
                && (self.all_projects || !self.project_ids.is_empty()))
            || (!matches!(self.role, MemberRole::User | MemberRole::Supervisor)
                && !self.all_projects
                && self.project_ids.is_empty())
            || self.project_ids.iter().any(|id| id.is_empty())
        {
            return Err(PlatformError::Validation("invalid member role assignment"));
        }
        let unique: std::collections::BTreeSet<_> = self.project_ids.iter().collect();
        if unique.len() != self.project_ids.len() {
            return Err(PlatformError::Validation("duplicate role project"));
        }
        Ok(())
    }
}
