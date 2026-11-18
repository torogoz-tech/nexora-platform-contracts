use nexora_platform::{BacklogChange, HexColor, ProjectChange, TaskTypeChange, WorkflowGraph};
use serde_json::json;

#[test]
fn colors_are_exact_hexadecimal_values_normalized_for_storage() {
    for input in ["#a1B2c3", "#000000", "#FFFFFF"] {
        let color: HexColor = serde_json::from_value(json!(input)).unwrap();
        assert_eq!(
            serde_json::to_value(color).unwrap(),
            input.to_ascii_uppercase()
        );
    }
    for invalid in [
        "red",
        "#123",
        "123456",
        "#12345678",
        "#GG0000",
        " #112233",
        "#112233\n",
        "",
    ] {
        assert!(
            serde_json::from_value::<HexColor>(json!(invalid)).is_err(),
            "{invalid}"
        );
    }
}

#[test]
fn color_changes_distinguish_omission_clear_and_replacement() {
    for patch in [json!({}), json!({"color":null}), json!({"color":"#aabbcc"})] {
        let mut body = json!({"expected_revision":1,"reason":"Visual distinction","name":"Example","archived":false});
        body.as_object_mut()
            .unwrap()
            .extend(patch.as_object().unwrap().clone());
        let mut project_body = body.clone();
        project_body.as_object_mut().unwrap().remove("reason");
        let project: ProjectChange = serde_json::from_value(project_body).unwrap();
        let expected = patch.get("color").map(|v| {
            if v.is_string() {
                json!("#AABBCC")
            } else {
                v.clone()
            }
        });
        assert_eq!(
            serde_json::to_value(project).unwrap().get("color").cloned(),
            expected
        );
        body.as_object_mut().unwrap().remove("archived");
        let backlog: BacklogChange = serde_json::from_value(body.clone()).unwrap();
        let kind: TaskTypeChange = serde_json::from_value(body).unwrap();
        assert_eq!(
            serde_json::to_value(backlog).unwrap().get("color").cloned(),
            expected
        );
        assert_eq!(
            serde_json::to_value(kind).unwrap().get("color").cloned(),
            expected
        );
    }
}

#[test]
fn old_workflow_graphs_keep_their_serialized_shape_and_colored_states_roundtrip() {
    let graph = json!({"states":[{"id":"todo","name":"Todo","initial":true,"terminal":false}],"transitions":[{"id":"stay","name":"Stay","from":"todo","to":"todo","guards":[]}]});
    let old: WorkflowGraph = serde_json::from_value(graph.clone()).unwrap();
    assert_eq!(serde_json::to_value(old).unwrap(), graph);
    let mut colored = graph;
    colored["states"][0]["color"] = json!("#AABBCC");
    let flow: WorkflowGraph = serde_json::from_value(colored.clone()).unwrap();
    flow.validate().unwrap();
    assert_eq!(serde_json::to_value(flow).unwrap(), colored);
}

#[test]
fn omitted_create_colors_preserve_existing_idempotency_inputs() {
    let kind = json!({"code":"task","name":"Task","description":null});
    let typed: nexora_platform::TaskTypeCreate = serde_json::from_value(kind.clone()).unwrap();
    assert_eq!(serde_json::to_value(typed).unwrap(), kind);
    let backlog = json!({"name":"Delivery","description":null,"workflow_version_id":"wfv_example"});
    let typed: nexora_platform::BacklogCreate = serde_json::from_value(backlog.clone()).unwrap();
    assert_eq!(serde_json::to_value(typed).unwrap(), backlog);
}
