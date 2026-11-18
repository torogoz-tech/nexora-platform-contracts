use serde::{Deserialize, Serialize};

/// Published platform metadata. Runtime identifiers describe data; they grant no authority.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RepositoryBinding {
    pub project_id: String,
    pub repository_id: String,
    pub runtime_tenant_id: String,
    pub runtime_repository_id: String,
    pub snapshot_id: String,
    pub payload_hash: String,
    pub pinned_commit: String,
    pub source_generation: i64,
    pub job_id: String,
    pub attempt: i32,
    pub revision: i64,
    pub published_at: String,
}
