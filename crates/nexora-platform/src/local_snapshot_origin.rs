use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalSnapshotOrigin {
    pub source_id: String,
    pub authorization_revision: i64,
    pub publication_id: String,
}
