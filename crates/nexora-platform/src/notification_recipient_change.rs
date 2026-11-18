use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NotificationRecipientChange {
    pub email: String,
    pub assignments: bool,
    pub approvals: bool,
    pub expected_revision: i64,
}
