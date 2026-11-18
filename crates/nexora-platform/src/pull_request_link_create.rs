use crate::{DomainText, PlatformError};
use serde::{Deserialize, Serialize};
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PullRequestLinkCreate {
    pub kind: String,
    pub target_id: String,
    pub version_id: Option<String>,
    pub target_revision: i64,
    pub reason: String,
}
impl PullRequestLinkCreate {
    pub fn validate(&self) -> Result<(), PlatformError> {
        if !["task", "requirement"].contains(&self.kind.as_str())
            || self.target_id.is_empty()
            || self.target_id.len() > 128
            || self.target_revision < 1
            || (self.kind == "task" && self.version_id.is_some())
            || (self.kind == "requirement"
                && !self
                    .version_id
                    .as_ref()
                    .is_some_and(|v| !v.is_empty() && v.len() <= 128))
        {
            return Err(PlatformError::Validation("invalid pull request link"));
        }
        DomainText::required(&self.reason, 4000)?;
        Ok(())
    }
}
