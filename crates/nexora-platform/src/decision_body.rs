use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DecisionBody {
    pub context: String,
    pub decision: String,
    pub alternatives: Vec<String>,
    pub consequences: Vec<String>,
    pub source: String,
}
