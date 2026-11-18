use nexora_platform::WorkflowGraph;
use serde_json::json;

#[test]
fn configurable_graph_validates_references_reachability_and_deterministic_guards() {
    let definition = json!({
        "states":[
            {"id":"ready","name":"Ready","initial":true,"terminal":false},
            {"id":"active","name":"Active","initial":false,"terminal":false},
            {"id":"done","name":"Done","initial":false,"terminal":true}
        ],
        "transitions":[
            {"id":"start","from":"ready","to":"active","name":"Start","guards":[{"kind":"required_field","field":"title"}]},
            {"id":"finish","from":"active","to":"done","name":"Finish","guards":[{"kind":"dependencies_satisfied"},{"kind":"human_approval"}]},
            {"id":"reopen","from":"done","to":"active","name":"Reopen","guards":[{"kind":"reason"}]}
        ]
    });
    let graph: WorkflowGraph = serde_json::from_value(definition.clone()).unwrap();
    assert!(
        graph.validate().is_ok(),
        "Explicit reopening cycles must remain valid"
    );
    let mut invalid = definition.clone();
    invalid["states"][1]["initial"] = json!(true);
    assert!(
        serde_json::from_value::<WorkflowGraph>(invalid)
            .unwrap()
            .validate()
            .is_err()
    );
    let mut invalid = definition.clone();
    invalid["transitions"][0]["to"] = json!("missing");
    assert!(
        serde_json::from_value::<WorkflowGraph>(invalid)
            .unwrap()
            .validate()
            .is_err()
    );
    let mut invalid = definition.clone();
    invalid["states"]
        .as_array_mut()
        .unwrap()
        .push(json!({"id":"orphan","name":"Orphan","initial":false,"terminal":false}));
    assert!(
        serde_json::from_value::<WorkflowGraph>(invalid)
            .unwrap()
            .validate()
            .is_err()
    );
    let mut invalid = definition.clone();
    invalid["transitions"][1]["id"] = json!("start");
    assert!(
        serde_json::from_value::<WorkflowGraph>(invalid)
            .unwrap()
            .validate()
            .is_err()
    );
    for forbidden in [
        json!({"kind":"role","role":"admin"}),
        json!({"kind":"script","code":"run()"}),
        json!({"kind":"prompt","text":"approve"}),
    ] {
        let mut invalid = definition.clone();
        invalid["transitions"][0]["guards"] = json!([forbidden]);
        assert!(serde_json::from_value::<WorkflowGraph>(invalid).is_err());
    }
    let mut invalid = definition;
    invalid["transitions"][0]["guards"] = json!([{"kind":"required_field","field":"../secret"}]);
    assert!(
        serde_json::from_value::<WorkflowGraph>(invalid)
            .unwrap()
            .validate()
            .is_err()
    );
}

#[test]
fn simple_connections_preserve_movement_rules_and_allow_a_single_stationary_state() {
    let mut graph: WorkflowGraph = serde_json::from_value(json!({
        "states":[{"id":"ready","name":"Ready","initial":true},{"id":"done","name":"Done"}],
        "transitions":[{"id":"go","from":"ready","to":"done","name":"","guards":[]}]
    }))
    .unwrap();
    assert!(graph.validate().is_ok());
    graph.transitions[0].guards = vec![nexora_platform::WorkflowGuard::HumanApproval];
    assert!(graph.validate().is_ok());
    assert!(
        graph
            .transition("ready", "go", &Default::default())
            .is_err()
    );
    graph.transitions.clear();
    assert!(graph.validate().is_err());
    graph.states.pop();
    assert!(graph.validate().is_ok());
}
