use crate::DocumentHistoryEvent;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentHistoryPage {
    pub items: Vec<DocumentHistoryEvent>,
    pub next_after: Option<i64>,
}
