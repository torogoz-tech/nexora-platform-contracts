use nexora_platform::{DocumentContent, WorkflowResourceKind};
use serde_json::json;

#[test]
fn practices_and_debt_have_distinct_typed_content_and_roundtrip() {
    for value in [
        json!({"kind":"best_practice","body":{"guideline":"Prefer explicit ownership","rationale":"Keep changes reviewable","examples":["One responsibility per module"],"source":"Team review"}}),
        json!({"kind":"technical_debt","body":{"problem":"Duplicated policy checks","impact":"Divergent authorization decisions","remediation":"Consolidate the shared policy","source":"Repository review"}}),
    ] {
        let content: DocumentContent = serde_json::from_value(value.clone()).unwrap();
        assert_ne!(content.resource_kind(), WorkflowResourceKind::Document);
        assert_eq!(serde_json::to_value(&content).unwrap(), value);
        let mut invalid = value;
        invalid["body"]["unrecognized"] = json!("never accepted");
        assert!(serde_json::from_value::<DocumentContent>(invalid).is_err());
    }
}
