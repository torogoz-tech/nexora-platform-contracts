use crate::{RepositoryJobKind, RepositoryJobState};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryJob {
    pub id: String,
    pub project_id: String,
    pub repository_id: String,
    /// For webhook jobs this is the source-consent owner, not the commit author.
    pub requested_by: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_event_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analysis_profile_version_id: Option<String>,
    pub kind: RepositoryJobKind,
    pub state: RepositoryJobState,
    pub revision: i64,
    pub attempt: i32,
    pub repository_revision: i64,
    pub source_generation: i64,
    pub pinned_commit: Option<String>,
    pub snapshot_id: Option<String>,
    pub error_code: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub finished_at: Option<String>,
}
