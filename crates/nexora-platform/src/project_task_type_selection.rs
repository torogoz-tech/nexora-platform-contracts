use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectTaskTypeSelection {
    pub codes: Vec<String>,
    pub revision: i64,
}
