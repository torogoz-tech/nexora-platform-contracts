use crate::GitProvider;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct RemoteRepositoryMetadata {
    pub provider: GitProvider,
    pub remote_id: String,
    pub full_name: String,
}
