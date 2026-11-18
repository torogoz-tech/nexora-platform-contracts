use crate::PlatformError;
use serde::{Deserialize, Serialize};
#[derive(Clone, Deserialize, Serialize)]
pub struct EmailAccessSettings {
    pub public_web_url: String,
    pub revision: i64,
}
impl EmailAccessSettings {
    pub fn validate(value: &str) -> Result<String, PlatformError> {
        if value.trim().is_empty() {
            return Ok(String::new());
        }
        let url = url::Url::parse(value.trim())
            .map_err(|_| PlatformError::Validation("invalid public web URL"))?;
        let local = matches!(url.host_str(), Some("localhost" | "127.0.0.1" | "[::1]"));
        if url.host_str().is_none()
            || !(url.scheme() == "https" || url.scheme() == "http" && local)
            || !url.username().is_empty()
            || url.password().is_some()
            || url.query().is_some()
            || url.fragment().is_some()
            || url.path() != "/"
            || value.len() > 2048
        {
            return Err(PlatformError::Validation(
                "public web URL must be an HTTPS origin",
            ));
        }
        Ok(url.as_str().trim_end_matches('/').to_owned())
    }
}
