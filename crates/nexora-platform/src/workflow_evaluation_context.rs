use serde_json::{Map, Value};

/// Trusted facts assembled by the application service from the resource and its
/// authorized operation. Deliberately not deserializable from a client request.
#[derive(Default)]
pub struct WorkflowEvaluationContext {
    pub fields: Map<String, Value>,
    pub assignee_valid: bool,
    pub resolution: Option<String>,
    pub reason: Option<String>,
    pub dependencies_satisfied: bool,
    pub evidence_valid: bool,
    pub approved_document_version: bool,
    pub human_approval_valid: bool,
    pub working_branch_reported: bool,
}
