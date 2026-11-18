use crate::{
    BestPracticeBody, BusinessRuleBody, DecisionBody, ProjectDocumentBody, RequirementBody,
    TechnicalDebtBody, WorkflowResourceKind,
};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    content = "body",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum DocumentContent {
    Requirement(RequirementBody),
    BusinessRule(BusinessRuleBody),
    Decision(DecisionBody),
    Document(ProjectDocumentBody),
    BestPractice(BestPracticeBody),
    TechnicalDebt(TechnicalDebtBody),
}
impl DocumentContent {
    pub fn resource_kind(&self) -> WorkflowResourceKind {
        match self {
            Self::Requirement(_) => WorkflowResourceKind::Requirement,
            Self::BusinessRule(_) => WorkflowResourceKind::BusinessRule,
            Self::Decision(_) => WorkflowResourceKind::Decision,
            Self::Document(_) => WorkflowResourceKind::Document,
            Self::BestPractice(_) => WorkflowResourceKind::BestPractice,
            Self::TechnicalDebt(_) => WorkflowResourceKind::TechnicalDebt,
        }
    }
}
