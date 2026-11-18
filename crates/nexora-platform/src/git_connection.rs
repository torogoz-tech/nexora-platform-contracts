use crate::{GitConnectionStatus, GitProvider};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GitConnection {
    pub id: String,
    pub provider: GitProvider,
    pub host: String,
    pub owner_actor_id: String,
    pub remote_account_id: String,
    pub remote_login: String,
    #[serde(default)]
    pub installation_id: Option<String>,
    pub status: GitConnectionStatus,
    pub revision: i64,
    pub expires_at: String,
    pub created_at: String,
    pub updated_at: String,
}
