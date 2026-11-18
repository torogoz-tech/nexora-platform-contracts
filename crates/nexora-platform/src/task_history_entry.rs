use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskHistoryEntry {
    pub id: String,
    pub project_id: Option<String>,
    pub task_id: String,
    pub sequence: i64,
    pub revision: i64,
    pub operation: String,
    pub actor_id: String,
    pub credential_id: String,
    pub details: serde_json::Value,
    pub created_at: String,
}
