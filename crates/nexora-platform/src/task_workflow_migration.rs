use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TaskWorkflowMigration {
    pub expected_revision: i64,
    pub workflow_version_id: String,
    pub state_mapping: BTreeMap<String, String>,
    pub reason: String,
}
