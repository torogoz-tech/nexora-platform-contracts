#[derive(serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskProjectChange {
    pub expected_revision: i64,
    #[serde(deserialize_with = "crate::nullable_project::deserialize_nullable_project")]
    pub project_id: Option<String>,
    pub reason: String,
}
