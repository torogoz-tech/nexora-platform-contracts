use nexora_platform::WorkflowGraph;
use serde_json::json;
fn graph() -> serde_json::Value {
    json!({"states":[{"id":"a","name":"A","initial":true,"progress_order":0,"prevent_backtracking":true},{"id":"b","name":"B","progress_order":1,"required_base_branch":"develop"}],"transitions":[{"id":"next","from":"a","to":"b","steps":[{"kind":"verify_push","repository_id":"repo"},{"kind":"advance_state"},{"kind":"notify","channel":"email"}]}]})
}
#[test]
fn missing_name_defaults_and_legacy_graph_still_roundtrips() {
    let value = graph();
    let graph: WorkflowGraph = serde_json::from_value(value).unwrap();
    assert_eq!(graph.transitions[0].name, "");
    assert!(graph.validate().is_ok());
}
#[test]
fn backwards_uses_explicit_order_and_never_coordinates() {
    let mut value = graph();
    value["states"][0]["progress_order"] = json!(2);
    assert!(
        serde_json::from_value::<WorkflowGraph>(value)
            .unwrap()
            .validate()
            .is_err()
    );
}
#[test]
fn ordered_steps_reject_unsafe_order_and_invalid_refs() {
    for steps in [
        json!([{"kind":"notify","channel":"email"},{"kind":"advance_state"}]),
        json!([{"kind":"advance_state"},{"kind":"require_approval"}]),
        json!([{"kind":"advance_state"},{"kind":"advance_state"}]),
        json!([{"kind":"verify_push","repository_id":"repo"}]),
        json!([{"kind":"create_pull_request","repository_id":"repo","source_branch":"develop","target_branch":"develop"},{"kind":"advance_state"}]),
    ] {
        let mut value = graph();
        value["transitions"][0]["steps"] = steps;
        assert!(
            serde_json::from_value::<WorkflowGraph>(value)
                .unwrap()
                .validate()
                .is_err()
        );
    }
}

#[test]
fn progress_order_matches_the_editor_supported_range() {
    for order in [-1, 65536] {
        let mut value = graph();
        value["states"][0]["progress_order"] = json!(order);
        assert!(
            serde_json::from_value::<WorkflowGraph>(value)
                .unwrap()
                .validate()
                .is_err()
        );
    }
}
