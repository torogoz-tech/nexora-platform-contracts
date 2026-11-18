use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OidcProvider {
    pub id: String,
    pub name: String,
    pub issuer: String,
    pub client_id: String,
    pub redirect_uri: String,
    pub enabled: bool,
    pub revision: i64,
    pub created_at: String,
    pub updated_at: String,
}
