use serde::{Deserialize, Serialize};
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrganizationDocumentWrite {
    pub kind: String,
    pub title: String,
    pub body: String,
    pub expected_revision: Option<i64>,
}
