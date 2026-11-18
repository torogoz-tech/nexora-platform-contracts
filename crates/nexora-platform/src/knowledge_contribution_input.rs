use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct KnowledgeContributionInput {
    pub project_id: Option<String>,
    pub kind: String,
    pub title: String,
    #[serde(default)]
    pub body: String,
    pub filename: Option<String>,
    pub data_base64: Option<String>,
    pub expected_revision: Option<i64>,
}
