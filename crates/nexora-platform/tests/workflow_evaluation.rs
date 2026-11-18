use nexora_platform::{WorkflowEvaluationContext, WorkflowGraph};
use serde_json::json;

#[test]
fn guards_use_trusted_context_and_human_approval_only_when_declared() {
    let mut graph:WorkflowGraph=serde_json::from_value(json!({
        "states":[{"id":"ready","name":"Ready","initial":true},{"id":"done","name":"Done","terminal":true}],
        "transitions":[{"id":"finish","name":"Finish","from":"ready","to":"done","guards":[
            {"kind":"required_field","field":"title"},{"kind":"assignee"},{"kind":"resolution"},
            {"kind":"reason"},{"kind":"dependencies_satisfied"},{"kind":"evidence_valid"},
            {"kind":"approved_document_version"},{"kind":"human_approval"}]}]
    })).unwrap();
    let mut context = WorkflowEvaluationContext::default();
    assert!(graph.transition("ready", "finish", &context).is_err());
    context.fields.insert("title".into(), json!("Delivery"));
    context
        .fields
        .insert("human_approval_valid".into(), json!(true));
    context.assignee_valid = true;
    context.resolution = Some("Resolved".into());
    context.reason = Some("Verified work".into());
    context.dependencies_satisfied = true;
    context.evidence_valid = true;
    context.approved_document_version = true;
    assert!(
        graph.transition("ready", "finish", &context).is_err(),
        "Subject data cannot invent trusted approval"
    );
    context.human_approval_valid = true;
    assert_eq!(
        graph.transition("ready", "finish", &context).unwrap().to,
        "done"
    );
    assert!(graph.transition("done", "finish", &context).is_err());
    assert!(graph.transition("ready", "unknown", &context).is_err());
    context.reason = Some("  ".into());
    assert!(graph.transition("ready", "finish", &context).is_err());
    graph.transitions[0].guards.clear();
    assert!(
        graph
            .transition("ready", "finish", &WorkflowEvaluationContext::default())
            .is_ok(),
        "No unconditional actor-kind or human-approval restriction"
    );
    graph.transitions[0].guards =
        serde_json::from_value(json!([{"kind":"required_field","field":"verified"}])).unwrap();
    let mut context = WorkflowEvaluationContext::default();
    context.fields.insert("verified".into(), json!(false));
    assert!(
        graph.transition("ready", "finish", &context).is_ok(),
        "A present boolean false is data, not a missing field"
    );
}
