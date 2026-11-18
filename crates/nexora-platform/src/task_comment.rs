use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskComment {
    pub id: String,
    pub project_id: Option<String>,
    pub task_id: String,
    pub sequence: i64,
    pub body: String,
    pub actor_id: String,
    #[serde(default)]
    pub actor_name: Option<String>,
    pub credential_id: String,
    pub created_at: String,
}
