use crate::{DomainText, PlatformError};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Operator-only input; never accepted by a remote repository endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LocalSnapshotAdoption {
    pub name: String,
    pub root: PathBuf,
    pub runtime_tenant_id: String,
    pub snapshot_id: String,
    pub expected_payload_hash: String,
    pub reason: String,
    #[serde(default)]
    pub repository_id: Option<String>,
    #[serde(default)]
    pub expected_revision: Option<i64>,
}
impl LocalSnapshotAdoption {
    pub fn validate(&self) -> Result<(), PlatformError> {
        DomainText::name(&self.name)?;
        DomainText::required(&self.reason, 4000)?;
        if [&self.runtime_tenant_id, &self.snapshot_id]
            .into_iter()
            .any(|v| v.is_empty() || v.len() > 4096 || v.chars().any(char::is_control))
            || self.expected_payload_hash.len() != 64
            || !self
                .expected_payload_hash
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
            || self.repository_id.is_some() != self.expected_revision.is_some()
            || self.expected_revision.is_some_and(|r| r < 1)
            || self.repository_id.as_ref().is_some_and(|id| {
                id.len() != 37
                    || !id.starts_with("repo_")
                    || !id[5..].bytes().all(|b| b.is_ascii_hexdigit())
            })
        {
            return Err(PlatformError::Validation("invalid local snapshot adoption"));
        }
        Ok(())
    }
}
