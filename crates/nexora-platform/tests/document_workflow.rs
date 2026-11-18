use nexora_platform::WorkflowGraph;
use serde_json::json;

#[test]
fn publication_eligibility_is_explicit_and_old_workflow_encoding_stays_unchanged() {
    let old = json!({"states":[{"id":"draft","name":"Draft","initial":true,"terminal":false},{"id":"reviewed","name":"Reviewed","initial":false,"terminal":true}],"transitions":[{"id":"review","name":"Review","from":"draft","to":"reviewed","guards":[]}]});
    let old_graph: WorkflowGraph = serde_json::from_value(old.clone()).unwrap();
    old_graph.validate().unwrap();
    assert!(
        serde_json::to_value(&old_graph).unwrap() == old,
        "existing workflow payloads must keep their published checksum shape"
    );
    let mut configured = old;
    configured["states"][1]["publishable"] = json!(true);
    let graph: WorkflowGraph = serde_json::from_value(configured)
        .expect("publication eligibility belongs to explicit flow configuration");
    graph.validate().unwrap();
    let encoded = serde_json::to_value(graph).unwrap();
    assert_eq!(encoded["states"][1]["publishable"], true);
    assert!(
        encoded["states"][0].get("publishable").is_none(),
        "default false must not alter historical graph encoding"
    );
}
