use crate::GitConnection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GitConnectionPage {
    pub items: Vec<GitConnection>,
    pub next_cursor: Option<String>,
}
