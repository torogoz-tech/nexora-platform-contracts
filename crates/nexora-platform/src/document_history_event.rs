use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentHistoryEvent {
    pub id: i64,
    pub project_id: String,
    pub document_id: String,
    pub version_id: String,
    pub event: String,
    pub actor_id: String,
    pub credential_id: String,
    pub reason: String,
    pub details: Value,
    pub created_at: String,
}
