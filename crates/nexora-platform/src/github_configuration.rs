use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GithubConfiguration {
    pub app_id: String,
    pub client_id: String,
    pub slug: String,
    pub origin: String,
    pub api_origin: String,
    pub host: String,
    pub redirect_uri: String,
    pub enabled: bool,
    pub revision: i64,
}
