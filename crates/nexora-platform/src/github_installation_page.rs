use crate::GithubInstallation;
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct GithubInstallationPage {
    pub installation_url: String,
    pub items: Vec<GithubInstallation>,
    pub next_page: Option<u32>,
}
