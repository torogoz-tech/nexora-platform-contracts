use nexora_platform::RepositoryAnalysisProfileChange;
use serde_json::json;
#[test]
fn legacy_assignment_hashes_and_new_modes_are_compatible() {
    let old = json!({"expected_revision":1,"profile_version_id":null,"reason":"Fallback"});
    let input: RepositoryAnalysisProfileChange = serde_json::from_value(old.clone()).unwrap();
    assert_eq!(serde_json::to_value(input).unwrap(), old);
    let automatic:RepositoryAnalysisProfileChange=serde_json::from_value(json!({"expected_revision":0,"mode":"automatic","profile_version_ids":[],"reason":"Automatic"})).unwrap();
    assert!(automatic.profile_version_id.is_none());
    assert_eq!(automatic.mode.as_deref(), Some("automatic"));
}
