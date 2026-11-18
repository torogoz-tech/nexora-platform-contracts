use crate::LocalSnapshotOrigin;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentEvidence {
    pub repository_id: String,
    pub snapshot_id: String,
    pub pinned_commit: Option<String>,
    pub payload_hash: String,
    pub path: String,
    pub start_line: usize,
    pub line_count: usize,
    pub context_checksum: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_origin: Option<LocalSnapshotOrigin>,
}
