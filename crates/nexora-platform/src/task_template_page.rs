use crate::TaskTemplate;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskTemplatePage {
    pub items: Vec<TaskTemplate>,
    pub next_after: Option<String>,
}
