use crate::LocalSourceState;
use serde::{Deserialize, Serialize};

/// Operator-attested source metadata. Filesystem roots are never exposed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalRepositorySourceInfo {
    pub source_id: String,
    pub source_revision: i64,
    pub authorization_revision: i64,
    pub source_generation: i64,
    pub state: LocalSourceState,
    pub owner_actor_id: String,
    pub binding_revision: i64,
    pub publication_id: String,
    pub snapshot_id: String,
    pub payload_hash: String,
}
