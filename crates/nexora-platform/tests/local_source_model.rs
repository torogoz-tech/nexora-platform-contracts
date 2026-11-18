use nexora_platform::{ProjectRepository, RepositoryRegistration, RepositorySourceKind};
use serde_json::{Value, json};

#[test]
fn source_model_preserves_git_json_and_distinguishes_local_snapshot_metadata() {
    let git = json!({
        "id":"repo_0123456789abcdef0123456789abcdef",
        "project_id":"prj_0123456789abcdef0123456789abcdef",
        "provider":"github","host":"github.com","requested_full_name":"fixture/source",
        "remote_id":null,"remote_full_name":null,"connection_id":null,
        "authorization_status":"awaiting_authorization","archived":false,"revision":1,
        "source_generation":0,"latest_source_commit":null,"source_changed_at":null,
        "created_at":"2026-11-04T00:00:00Z","updated_at":"2026-11-04T00:00:00Z"
    });
    let original: ProjectRepository = serde_json::from_value(git.clone()).unwrap();
    assert_eq!(original.source_kind, RepositorySourceKind::Git);
    assert_eq!(original.provider.unwrap().as_str(), "github");
    assert_eq!(serde_json::to_value(&original).unwrap(), git);

    let mut local = git.clone();
    local["provider"] = Value::Null;
    local["host"] = Value::Null;
    local["requested_full_name"] = Value::Null;
    local["source_kind"] = json!("local_snapshot");
    local["local_name"] = json!("Imported source");
    local["authorization_status"] = json!("authorized");
    local["local_source"] = json!({
        "source_id":"lsrc_0123456789abcdef0123456789abcdef",
        "source_revision":1,"authorization_revision":1,"source_generation":0,
        "state":"active","owner_actor_id":"act_0123456789abcdef0123456789abcdef",
        "binding_revision":1,"publication_id":"lpub_0123456789abcdef0123456789abcdef",
        "snapshot_id":"original-snapshot","payload_hash":"a".repeat(64)
    });
    let adopted: ProjectRepository = serde_json::from_value(local.clone()).unwrap();
    assert_eq!(adopted.source_kind, RepositorySourceKind::LocalSnapshot);
    assert!(adopted.provider.is_none());
    assert!(adopted.host.is_none());
    assert!(adopted.requested_full_name.is_none());
    assert!(adopted.remote_id.is_none());
    assert!(adopted.connection_id.is_none());
    assert!(adopted.latest_source_commit.is_none());
    assert_eq!(serde_json::to_value(&adopted).unwrap(), local);

    // The remote registration contract remains a Git provider locator, never an operator root.
    assert!(
        serde_json::from_value::<RepositoryRegistration>(json!({
            "provider":"github","full_name":"fixture/source","source_kind":"local_snapshot"
        }))
        .is_err()
    );
    assert!(
        serde_json::from_value::<RepositoryRegistration>(json!({
            "provider":"local_snapshot","full_name":"fixture/source","root":"/client/root"
        }))
        .is_err()
    );
}
