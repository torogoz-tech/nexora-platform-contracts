use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct GithubConsent {
    pub id: String,
    pub remote_account_id: String,
    pub remote_login: String,
    pub expires_at: String,
}
