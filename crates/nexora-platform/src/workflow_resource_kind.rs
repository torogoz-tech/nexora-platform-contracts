use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowResourceKind {
    Task,
    Requirement,
    BusinessRule,
    Decision,
    Document,
    BestPractice,
    TechnicalDebt,
}
