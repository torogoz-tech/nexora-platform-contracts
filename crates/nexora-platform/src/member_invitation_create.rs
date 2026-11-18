use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MemberInvitationCreate {
    pub email: String,
    pub name: String,
}
