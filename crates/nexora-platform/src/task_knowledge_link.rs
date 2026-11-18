#[derive(serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskKnowledgeLink {
    pub kind: String,
    pub target_id: String,
    pub expected_revision: i64,
    pub expected_target_revision: i64,
    pub version_id: Option<String>,
    pub reason: String,
}
