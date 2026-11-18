use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GitlabConfiguration {
    pub origin: String,
    pub host: String,
    pub client_id: String,
    pub redirect_uri: String,
    pub enabled: bool,
    pub revision: i64,
}
