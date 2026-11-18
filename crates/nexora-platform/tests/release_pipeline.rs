use nexora_platform::WorkflowGraph;
use serde_json::{Value, json};
fn graph(steps: Value) -> WorkflowGraph {
    serde_json::from_value(json!({"states":[{"id":"ready","name":"Ready","initial":true},{"id":"done","name":"Fin","terminal":true}],"transitions":[{"id":"finish","name":"Finish","from":"ready","to":"done","guards":[]},{"id":"publish","name":"Publish","from":"done","to":"done","automatic":true,"guards":[],"steps":steps}]})).unwrap()
}
#[test]
fn release_publication_is_the_last_approved_terminal_step_after_verified_production() {
    let valid = json!([{"kind":"require_approval"},{"kind":"merge_pull_request","target_branch":"main"},{"kind":"verify_deployment","environment":"production","target_branch":"main"},{"kind":"advance_state"},{"kind":"publish_release"}]);
    assert!(graph(valid.clone()).validate().is_ok());
    let mut missing_production = valid.clone();
    missing_production[2]["environment"] = json!("preproduction");
    assert!(graph(missing_production).validate().is_err());
    let mut no_approval = valid.as_array().unwrap().clone();
    no_approval.remove(0);
    assert!(graph(json!(no_approval)).validate().is_err());
    let mut not_final = valid.as_array().unwrap().clone();
    not_final.push(json!({"kind":"notify","channel":"in_app"}));
    assert!(graph(json!(not_final)).validate().is_err());
}
