use serde::{Deserialize, Serialize};

/// Public identity fields for authorized resource presentations, without profile activity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberIdentitySummary {
    pub id: String,
    pub name: String,
    pub position: String,
    pub avatar_revision: i64,
}
