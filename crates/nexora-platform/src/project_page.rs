use crate::Project;
use serde::{Deserialize, Serialize};
/// Bounded traversal in ID order; does not promise a snapshot across requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectPage {
    pub items: Vec<Project>,
    pub next_cursor: Option<String>,
}
