use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct MemberInvitation {
    pub id: String,
    pub email: String,
    pub name: String,
    pub status: String,
    pub revision: i64,
    pub expires_at: String,
    pub created_at: String,
}
