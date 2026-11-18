use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct GithubInstallation {
    pub id: String,
    pub account_id: String,
    pub account_login: String,
    pub repository_selection: String,
}
