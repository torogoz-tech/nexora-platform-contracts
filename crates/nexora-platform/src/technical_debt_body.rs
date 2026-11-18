use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TechnicalDebtBody {
    pub problem: String,
    pub impact: String,
    pub remediation: String,
    pub source: String,
}
