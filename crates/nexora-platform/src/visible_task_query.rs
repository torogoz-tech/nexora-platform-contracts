use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct VisibleTaskQuery {
    pub descending: bool,
    pub type_code: Option<String>,
    pub workflow_id: Option<String>,
    pub project_id: Option<String>,
    pub assignee_id: Option<String>,
    pub search: Option<String>,
    pub lifecycle: Option<crate::TaskLifecycle>,
    pub after: Option<String>,
    pub limit: u16,
}
impl Default for VisibleTaskQuery {
    fn default() -> Self {
        Self {
            descending: false,
            type_code: None,
            workflow_id: None,
            project_id: None,
            search: None,
            assignee_id: None,
            lifecycle: None,
            after: None,
            limit: 50,
        }
    }
}
