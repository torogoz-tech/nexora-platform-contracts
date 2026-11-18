use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GithubManifestRequest {
    pub name: String,
    pub expected_revision: i64,
    #[serde(default)]
    pub organization: Option<String>,
    #[serde(default)]
    pub public: bool,
}
