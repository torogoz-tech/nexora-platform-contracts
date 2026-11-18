use crate::VisibleTaskType;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisibleTaskTypePage {
    pub items: Vec<VisibleTaskType>,
    pub next_after: Option<String>,
    pub total: i64,
}
