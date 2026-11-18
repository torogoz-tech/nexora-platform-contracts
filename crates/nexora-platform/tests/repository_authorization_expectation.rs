use nexora_platform::GitRepositoryAuthorization;
use serde_json::json;

#[test]
fn selected_identity_expectations_preserve_legacy_receipt_inputs() {
    let legacy = json!({"connection_id":"gcn_fixture","expected_revision":1});
    let authorization: GitRepositoryAuthorization = serde_json::from_value(legacy.clone()).unwrap();
    assert_eq!(serde_json::to_value(authorization).unwrap(), legacy);
    let selected =
        json!({"connection_id":"gcn_fixture","expected_revision":1,"expected_remote_id":"101"});
    let authorization = serde_json::from_value::<GitRepositoryAuthorization>(selected.clone());
    assert!(
        authorization.is_ok(),
        "Remote identity selection expectation is unsupported"
    );
    assert_eq!(
        serde_json::to_value(authorization.unwrap()).unwrap(),
        selected
    );
}
