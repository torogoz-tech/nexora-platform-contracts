use serde::{Deserialize, Serialize};

/// Declarative checks only. No roles, scripts, prompts or arbitrary requests.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum WorkflowGuard {
    RequiredField { field: String },
    Assignee,
    Resolution,
    Reason,
    DependenciesSatisfied,
    EvidenceValid,
    ApprovedDocumentVersion,
    HumanApproval,
    WorkingBranchReported,
}
