use nexora_platform::{DocumentEvidence, ProjectContextMember};
use serde_json::{Value, json};

#[test]
fn optional_local_provenance_preserves_the_exact_git_member_and_evidence_json() {
    let member = json!({"repository_id":"repo_fixture","snapshot_id":"snapshot",
        "payload_hash":"a".repeat(64),"pinned_commit":"b".repeat(40),"source_generation":0,
        "connection_id":"conn_fixture","connection_revision":1,"binding_revision":1});
    let parsed: ProjectContextMember = serde_json::from_value(member.clone()).unwrap();
    assert_eq!(serde_json::to_value(parsed).unwrap(), member);
    let evidence = json!({"repository_id":"repo_fixture","snapshot_id":"snapshot","pinned_commit":"b".repeat(40),
        "payload_hash":"a".repeat(64),"path":"source.js","start_line":1,"line_count":1,"context_checksum":"c".repeat(64)});
    let parsed: DocumentEvidence = serde_json::from_value(evidence.clone()).unwrap();
    assert_eq!(serde_json::to_value(parsed).unwrap(), evidence);
    let origin = json!({"source_id":"lsrc_fixture","authorization_revision":1,"publication_id":"lpub_fixture"});
    let mut local = member;
    local["pinned_commit"] = Value::Null;
    local["connection_id"] = Value::Null;
    local["connection_revision"] = Value::Null;
    local["local_origin"] = origin.clone();
    let parsed: ProjectContextMember = serde_json::from_value(local.clone()).unwrap();
    assert_eq!(serde_json::to_value(parsed).unwrap(), local);
    let mut local = evidence;
    local["pinned_commit"] = Value::Null;
    local["local_origin"] = origin;
    let parsed: DocumentEvidence = serde_json::from_value(local.clone()).unwrap();
    assert_eq!(serde_json::to_value(parsed).unwrap(), local);
}
