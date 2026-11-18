use crate::LocalSnapshotOrigin;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectContextMember {
    pub repository_id: String,
    pub snapshot_id: String,
    pub payload_hash: String,
    pub pinned_commit: Option<String>,
    pub source_generation: i64,
    pub connection_id: Option<String>,
    pub connection_revision: Option<i64>,
    pub binding_revision: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_origin: Option<LocalSnapshotOrigin>,
}
