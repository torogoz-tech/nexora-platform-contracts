use crate::{DocumentTransitionRequest, TaskTransitionRequest};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ApprovalRequestTarget {
    Task {
        task_id: String,
        transition_id: String,
        parameters: TaskTransitionRequest,
    },
    DocumentVersion {
        document_id: String,
        version_id: String,
        transition_id: String,
        parameters: DocumentTransitionRequest,
    },
}
