use crate::DocumentPublication;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentPublicationPage {
    pub items: Vec<DocumentPublication>,
    pub next_after: Option<String>,
}
