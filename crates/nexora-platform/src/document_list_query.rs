use crate::WorkflowResourceKind;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct DocumentListQuery {
    pub after: Option<String>,
    pub limit: u16,
    pub kind: Option<WorkflowResourceKind>,
    pub include_archived: bool,
}
impl Default for DocumentListQuery {
    fn default() -> Self {
        Self {
            after: None,
            limit: 50,
            kind: None,
            include_archived: false,
        }
    }
}
