use serde::Serialize;
use serde_json::Value;
#[derive(Serialize)]
pub struct GithubManifestAuthorization {
    pub action_url: String,
    pub manifest: Value,
    pub state_id: String,
    #[serde(skip_serializing)]
    pub correlation_secret: String,
    #[serde(skip_serializing)]
    pub secure_cookie: bool,
}
