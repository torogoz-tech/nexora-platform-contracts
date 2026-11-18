use crate::MemberRole;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemberRoleAssignment {
    pub role: Option<MemberRole>,
    pub all_projects: bool,
    pub project_ids: Vec<String>,
    pub revision: i64,
}
