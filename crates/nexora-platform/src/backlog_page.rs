use crate::Backlog;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BacklogPage {
    pub items: Vec<Backlog>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_after: Option<String>,
}
