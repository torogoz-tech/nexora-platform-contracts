use serde::{Deserialize, Serialize};
#[derive(Clone, Deserialize, Serialize)]
pub struct NotificationRecipient {
    pub actor_id: String,
    pub name: String,
    pub email: String,
    pub assignments: bool,
    pub approvals: bool,
    pub revision: i64,
}
