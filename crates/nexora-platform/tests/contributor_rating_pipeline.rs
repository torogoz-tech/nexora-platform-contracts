use nexora_platform::WorkflowGraph;
use serde_json::{Value, json};
fn graph(steps: Value) -> WorkflowGraph {
    serde_json::from_value(json!({"states":[{"id":"ready","name":"Ready","initial":true},{"id":"done","name":"Fin","terminal":true}],"transitions":[{"id":"finish","name":"Finish","from":"ready","to":"done","steps":steps}]})).unwrap()
}
#[test]
fn personal_assessment_is_explicit_after_matching_review_before_advance() {
    assert!(graph(json!([{"kind":"create_code_review"},{"kind":"generate_contributor_ratings"},{"kind":"advance_state"}])).validate().is_ok());
    for steps in [
        json!([{"kind":"generate_contributor_ratings"},{"kind":"advance_state"}]),
        json!([{"kind":"create_code_review","repository_id":"repo_other"},{"kind":"generate_contributor_ratings","repository_id":"repo_selected"},{"kind":"advance_state"}]),
        json!([{"kind":"create_code_review"},{"kind":"advance_state"},{"kind":"generate_contributor_ratings"}]),
    ] {
        assert!(graph(steps).validate().is_err());
    }
}
