use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GithubInstallationSelection {
    pub installation_id: String,
}
