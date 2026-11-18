use crate::RemoteRepositoryMetadata;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct RemoteRepositoryPage {
    pub items: Vec<RemoteRepositoryMetadata>,
    pub next_page: Option<u32>,
}
