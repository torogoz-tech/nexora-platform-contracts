use crate::DocumentSummary;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentPage {
    pub items: Vec<DocumentSummary>,
    pub next_after: Option<String>,
}
