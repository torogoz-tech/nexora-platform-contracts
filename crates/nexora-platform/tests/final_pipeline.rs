use nexora_platform::*;
use serde_json::json;
#[test]
fn automatic_pipeline_requires_terminal_source_and_single_exit() {
    let mut graph:WorkflowGraph=serde_json::from_value(json!({"states":[{"id":"start","name":"Start","initial":true},{"id":"end","name":"End","terminal":true}],"transitions":[{"id":"finish","from":"start","to":"end"},{"id":"deliver","from":"end","to":"end","automatic":true,"steps":[{"kind":"verify_pipeline"},{"kind":"advance_state"},{"kind":"handoff","workflow_id":"qa"}]}]})).unwrap();
    assert!(graph.validate().is_ok());
    graph.transitions[1].from = "start".into();
    assert!(graph.validate().is_err());
}
#[test]
fn handoff_cannot_be_manual_or_before_advancement() {
    let graph:WorkflowGraph=serde_json::from_value(json!({"states":[{"id":"start","name":"Start","initial":true},{"id":"end","name":"End","terminal":true}],"transitions":[{"id":"finish","from":"start","to":"end","steps":[{"kind":"handoff","workflow_id":"qa"},{"kind":"advance_state"}]}]})).unwrap();
    assert!(graph.validate().is_err());
}
