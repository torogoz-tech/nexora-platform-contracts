use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub color: Option<crate::HexColor>,
    pub description: Option<String>,
    pub archived: bool,
    pub revision: i64,
    pub created_at: String,
    pub updated_at: String,
}
