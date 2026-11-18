use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct GitAuthorQuery {
    pub project_id: Option<String>,
    pub member_id: Option<String>,
    pub search: Option<String>,
    pub after: Option<String>,
    pub limit: u16,
    pub unlinked_only: bool,
    pub archived_only: bool,
}
impl Default for GitAuthorQuery {
    fn default() -> Self {
        Self {
            project_id: None,
            member_id: None,
            search: None,
            after: None,
            limit: 50,
            unlinked_only: false,
            archived_only: false,
        }
    }
}
