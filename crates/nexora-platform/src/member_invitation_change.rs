use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MemberInvitationChange {
    pub expected_revision: i64,
}
