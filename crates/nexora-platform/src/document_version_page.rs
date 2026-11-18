use crate::DocumentVersionSummary;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentVersionPage {
    pub items: Vec<DocumentVersionSummary>,
    pub next_before: Option<i64>,
}
