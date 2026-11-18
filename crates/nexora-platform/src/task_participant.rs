use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskParticipant {
    pub actor_id: String,
    pub name: String,
    pub total: i64,
}
