use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GitRepositoryAuthorization {
    pub connection_id: String,
    pub expected_revision: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_remote_id: Option<String>,
}
