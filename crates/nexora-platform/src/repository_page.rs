use crate::ProjectRepository;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryPage {
    pub items: Vec<ProjectRepository>,
    pub next_cursor: Option<String>,
}
