use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogTaskTypePage {
    pub items: Vec<crate::CatalogTaskType>,
    pub next_after: Option<String>,
}
