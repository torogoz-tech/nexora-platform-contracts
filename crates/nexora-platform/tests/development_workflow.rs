use nexora_platform::{
    DevelopmentPolicy, WorkflowEvaluationContext, WorkflowGraph, WorkflowResourceKind,
};
use serde_json::json;
#[test]
fn branch_guard_uses_trusted_context_and_only_task_flows() {
    let graph:WorkflowGraph=serde_json::from_value(json!({"states":[{"id":"ready","name":"Ready","initial":true},{"id":"doing","name":"Doing","terminal":true}],"transitions":[{"id":"start","name":"Start","from":"ready","to":"doing","guards":[{"kind":"working_branch_reported"}]}]})).unwrap();
    let mut context = WorkflowEvaluationContext::default();
    context
        .fields
        .insert("working_branch_reported".into(), json!(true));
    assert!(graph.transition("ready", "start", &context).is_err());
    context.working_branch_reported = true;
    assert!(graph.transition("ready", "start", &context).is_ok());
    assert!(
        graph
            .validate_resource_kinds(&[WorkflowResourceKind::Task])
            .is_ok()
    );
    assert!(
        graph
            .validate_resource_kinds(&[WorkflowResourceKind::Document])
            .is_err()
    );
    for value in ["develop", "feature/task-12", "release/1.2"] {
        assert!(DevelopmentPolicy::valid_ref(value));
    }
    for value in [
        "", "-x", "a..b", "a.lock/x", "a/.x", "a@{x}", "a//b", "a;touch", "a b",
    ] {
        assert!(!DevelopmentPolicy::valid_ref(value), "{value}");
    }
}
