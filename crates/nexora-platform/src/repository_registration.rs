use crate::{GitProvider, PlatformError};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RepositoryRegistration {
    pub provider: GitProvider,
    pub full_name: String,
}
impl RepositoryRegistration {
    /// Validates a requested provider locator, never a filesystem path or authenticated identity.
    pub fn locator(&self) -> Result<&str, PlatformError> {
        let value = self.full_name.trim();
        if value.is_empty() || value.len() > 512 || self.full_name.chars().any(char::is_control) {
            return Err(PlatformError::Validation(
                "invalid requested repository name",
            ));
        }
        let parts: Vec<_> = value.split('/').collect();
        if parts.len() < 2
            || parts.len() > 20
            || (self.provider == GitProvider::Github && parts.len() != 2)
            || parts.iter().any(|p| {
                p.is_empty()
                    || *p == "."
                    || *p == ".."
                    || !p
                        .bytes()
                        .all(|b| b.is_ascii_alphanumeric() || b"._-".contains(&b))
            })
        {
            return Err(PlatformError::Validation(
                "expected provider namespace/repository, not a URL or server path",
            ));
        }
        Ok(value)
    }
}
