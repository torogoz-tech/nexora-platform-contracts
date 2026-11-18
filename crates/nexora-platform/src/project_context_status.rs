use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectContextStatus {
    pub manifest_id: String,
    pub needs_review: bool,
    pub changed_repository_ids: Vec<String>,
    pub unavailable_repository_ids: Vec<String>,
}
