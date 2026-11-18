use crate::DeveloperDirectoryEntry;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeveloperDirectoryPage {
    pub items: Vec<DeveloperDirectoryEntry>,
    pub next_after: Option<String>,
}
