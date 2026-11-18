use nexora_platform::*;
use serde_json::json;
#[test]
fn purposes_are_stable_values_independent_of_custom_type_codes() {
    for (purpose, value) in [
        (TaskPurpose::ProductDesign, "product_design"),
        (TaskPurpose::Implementation, "implementation"),
        (TaskPurpose::General, "general"),
    ] {
        assert_eq!(serde_json::to_value(purpose).unwrap(), json!(value));
        assert_eq!(purpose.as_str(), value);
    }
    assert!(serde_json::from_value::<TaskPurpose>(json!("my_custom_type")).is_err());
}
#[test]
fn primary_links_and_entry_workflow_are_explicit_creation_fields() {
    let input:TaskCreate=serde_json::from_value(json!({"title":"Design checkout", "type_code":"custom_design", "entry_workflow_id":"design_flow", "primary_requirement_id":"checkout_requirement"})).unwrap();
    assert_eq!(input.entry_workflow_id.as_deref(), Some("design_flow"));
    assert_eq!(
        input.primary_requirement_id.as_deref(),
        Some("checkout_requirement")
    );
    assert!(input.primary_feature_id.is_none());
}
#[test]
fn catalog_type_purpose_cannot_be_redefined_by_metadata_change() {
    assert!(
        serde_json::from_value::<CatalogTaskTypeChange>(
            json!({"expected_revision":1,"purpose":"implementation"})
        )
        .is_err()
    );
}
