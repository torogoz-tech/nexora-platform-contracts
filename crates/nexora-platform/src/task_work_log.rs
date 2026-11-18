use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskWorkLog {
    pub id: String,
    pub task_id: String,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub start_time: Option<String>,
    #[serde(default)]
    pub end_time: Option<String>,
    pub actor_id: String,
    #[serde(default)]
    pub actor_name: Option<String>,
    pub date: String,
    pub overtime_request_id: Option<String>,
    pub minutes: i32,
    pub note: Option<String>,
    pub revision: i64,
    pub created_at: String,
    pub updated_at: String,
}
