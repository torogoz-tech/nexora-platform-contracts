use crate::DeveloperBoardTask;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeveloperBoardPage {
    pub items: Vec<DeveloperBoardTask>,
    pub next_after: Option<String>,
}
