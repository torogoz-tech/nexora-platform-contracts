use crate::{
    GitProvider, LocalRepositorySourceInfo, RepositoryAuthorization, RepositorySourceKind,
};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectRepository {
    pub id: String,
    pub project_id: String,
    #[serde(default, skip_serializing_if = "RepositorySourceKind::is_git")]
    pub source_kind: RepositorySourceKind,
    pub provider: Option<GitProvider>,
    pub host: Option<String>,
    pub requested_full_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_source: Option<LocalRepositorySourceInfo>,
    pub remote_id: Option<String>,
    pub remote_full_name: Option<String>,
    pub connection_id: Option<String>,
    pub authorization_status: RepositoryAuthorization,
    pub archived: bool,
    pub revision: i64,
    #[serde(default)]
    pub source_generation: i64,
    pub latest_source_commit: Option<String>,
    pub source_changed_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
