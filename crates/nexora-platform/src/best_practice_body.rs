use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BestPracticeBody {
    pub guideline: String,
    pub rationale: String,
    #[serde(default)]
    pub examples: Vec<String>,
    pub source: String,
}
