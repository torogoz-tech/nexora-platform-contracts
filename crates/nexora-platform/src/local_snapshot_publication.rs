use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalSnapshotPublication {
    pub project_id: String,
    pub repository_id: String,
    pub source_id: String,
    pub publication_id: String,
    pub runtime_tenant_id: String,
    pub runtime_repository_id: String,
    pub snapshot_id: String,
    pub payload_hash: String,
    pub source_revision: i64,
    pub authorization_revision: i64,
    pub source_generation: i64,
    pub binding_revision: i64,
    pub attested_by: String,
    pub published_at: String,
}
