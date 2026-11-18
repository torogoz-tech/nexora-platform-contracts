use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct GitlabWebhook {
    pub id: String,
    pub repository_id: String,
    pub callback_url: String,
    pub active: bool,
    pub revision: i64,
    pub created_at: String,
}
