use crate::{PlatformError, PullRequestOperation};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PullRequestQuery {
    pub operation: PullRequestOperation,
    pub number: Option<u64>,
    pub state: String,
    pub page: u32,
    pub limit: u16,
}
impl PullRequestQuery {
    pub fn validate(&self) -> Result<(), PlatformError> {
        if !(1..=10000).contains(&self.page)
            || !(1..=100).contains(&self.limit)
            || !["open", "closed", "all"].contains(&self.state.as_str())
            || (self.operation != PullRequestOperation::List
                && !self.number.is_some_and(|v| v > 0 && v <= i64::MAX as u64))
        {
            return Err(PlatformError::Validation("invalid pull request query"));
        }
        Ok(())
    }
}
