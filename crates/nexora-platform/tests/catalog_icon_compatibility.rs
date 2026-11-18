use nexora_platform::{CatalogTaskTypeChange, CatalogTaskTypeCreate};
use serde_json::json;

#[test]
fn omitted_icons_preserve_legacy_catalog_request_fingerprints() {
    let create: CatalogTaskTypeCreate =
        serde_json::from_value(json!({"code":"compat","name":"Compat"})).unwrap();
    assert_eq!(
        serde_json::to_value(create).unwrap(),
        json!({"code":"compat","name":"Compat","description":null})
    );
    let change: CatalogTaskTypeChange =
        serde_json::from_value(json!({"expected_revision":1})).unwrap();
    assert_eq!(
        serde_json::to_value(change).unwrap(),
        json!({"expected_revision":1,"name":null,"description":null,"archived":null})
    );
    let create: CatalogTaskTypeCreate =
        serde_json::from_value(json!({"code":"compat","name":"Compat","icon":"bug"})).unwrap();
    assert_eq!(serde_json::to_value(create).unwrap()["icon"], "bug");
}
