use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepositoryJobKind {
    Sync,
    Index,
}
impl RepositoryJobKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sync => "sync",
            Self::Index => "index",
        }
    }
    pub fn operation(self) -> &'static str {
        match self {
            Self::Sync => "repository.sync",
            Self::Index => "repository.index",
        }
    }
}
