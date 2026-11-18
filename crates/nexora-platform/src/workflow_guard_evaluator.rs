use crate::{DomainText, PlatformError, WorkflowEvaluationContext, WorkflowGuard};
use serde_json::Value;

pub(crate) struct WorkflowGuardEvaluator;
impl WorkflowGuardEvaluator {
    pub fn evaluate(
        guards: &[WorkflowGuard],
        context: &WorkflowEvaluationContext,
    ) -> Result<(), PlatformError> {
        for guard in guards {
            let (satisfied, message) = match guard {
                WorkflowGuard::RequiredField { field } => (
                    context.fields.get(field).is_some_and(Self::present),
                    "required workflow field is empty",
                ),
                WorkflowGuard::Assignee => {
                    (context.assignee_valid, "workflow requires a valid assignee")
                }
                WorkflowGuard::Resolution => (
                    Self::text(context.resolution.as_deref()),
                    "workflow requires a resolution",
                ),
                WorkflowGuard::Reason => (
                    Self::text(context.reason.as_deref()),
                    "workflow requires a reason",
                ),
                WorkflowGuard::DependenciesSatisfied => (
                    context.dependencies_satisfied,
                    "workflow dependencies are not satisfied",
                ),
                WorkflowGuard::EvidenceValid => {
                    (context.evidence_valid, "workflow requires valid evidence")
                }
                WorkflowGuard::ApprovedDocumentVersion => (
                    context.approved_document_version,
                    "workflow requires an approved document version",
                ),
                WorkflowGuard::WorkingBranchReported => (
                    context.working_branch_reported,
                    "workflow requires a persisted agent-reported task branch",
                ),
                WorkflowGuard::HumanApproval => (
                    context.human_approval_valid,
                    "workflow requires valid human approval",
                ),
            };
            if !satisfied {
                return Err(PlatformError::Validation(message));
            }
        }
        Ok(())
    }
    fn text(value: Option<&str>) -> bool {
        value.is_some_and(|text| DomainText::required(text, 4000).is_ok())
    }
    fn present(value: &Value) -> bool {
        match value {
            Value::Null => false,
            Value::String(text) => !text.trim().is_empty(),
            Value::Array(values) => !values.is_empty(),
            Value::Object(values) => !values.is_empty(),
            Value::Bool(_) | Value::Number(_) => true,
        }
    }
}
