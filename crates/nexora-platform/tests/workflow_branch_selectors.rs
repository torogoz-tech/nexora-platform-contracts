use nexora_platform::{WorkflowGraph, WorkflowTransitionStep};
use serde_json::json;
#[test]
fn literal_branch_workflow_serialization_remains_unchanged_for_historical_versions() {
    let literal = json!({"kind":"create_pull_request","source_branch":"work/legacy","target_branch":"develop"});
    let step: WorkflowTransitionStep = serde_json::from_value(literal.clone()).unwrap();
    assert_eq!(serde_json::to_value(step).unwrap(), literal);
}
#[test]
fn reusable_task_and_parent_branch_selectors_are_explicit_and_validated() {
    let definition = json!({"states":[{"id":"ready","name":"Ready","initial":true},{"id":"done","name":"Done","terminal":true}],"transitions":[{"id":"finish","from":"ready","to":"done","steps":[
        {"kind":"create_pull_request","source_from_task":true,"target_from_parent":true},
        {"kind":"require_approval"},
        {"kind":"merge_pull_request","target_from_parent":true},
        {"kind":"verify_pull_request_merged","target_from_parent":true},
        {"kind":"verify_deployment","environment":"qa","target_from_parent":true},
        {"kind":"advance_state"}
    ]}]});
    let graph: WorkflowGraph = serde_json::from_value(definition.clone()).unwrap();
    graph.validate().unwrap();
    let mut ambiguous = definition.clone();
    ambiguous["transitions"][0]["steps"][0]["source_branch"] = json!("literal");
    assert!(
        serde_json::from_value::<WorkflowGraph>(ambiguous)
            .unwrap()
            .validate()
            .is_err()
    );
    let mut unnamed = definition;
    unnamed["transitions"][0]["steps"][0]["source_from_task"] = json!(false);
    assert!(
        serde_json::from_value::<WorkflowGraph>(unnamed)
            .unwrap()
            .validate()
            .is_err()
    );
}
