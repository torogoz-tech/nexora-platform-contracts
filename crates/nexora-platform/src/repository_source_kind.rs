use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepositorySourceKind {
    #[default]
    Git,
    LocalSnapshot,
}

impl RepositorySourceKind {
    pub fn is_git(&self) -> bool {
        *self == Self::Git
    }
}
