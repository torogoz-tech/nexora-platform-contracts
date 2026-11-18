use crate::DocumentLink;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentLinkPage {
    pub items: Vec<DocumentLink>,
    pub next_after: Option<String>,
}
