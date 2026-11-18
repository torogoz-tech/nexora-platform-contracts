use nexora_platform::{RepositoryJob, RepositoryJobKind, RepositoryJobState};
use serde_json::json;

#[test]
fn repository_job_wire_shape_preserves_current_fields_and_omits_absent_extensions() {
    let job = RepositoryJob {
        id: "job_123".into(),
        project_id: "project_123".into(),
        repository_id: "repository_123".into(),
        requested_by: "actor_123".into(),
        source_event_id: None,
        analysis_profile_version_id: None,
        kind: RepositoryJobKind::Index,
        state: RepositoryJobState::Queued,
        revision: 1,
        attempt: 0,
        repository_revision: 7,
        source_generation: 3,
        pinned_commit: None,
        snapshot_id: None,
        error_code: None,
        created_at: "2026-11-18T00:00:00Z".into(),
        updated_at: "2026-11-18T00:00:00Z".into(),
        finished_at: None,
    };

    let serialized = serde_json::to_value(&job).unwrap();

    assert_eq!(
        serialized,
        json!({
            "id": "job_123",
            "project_id": "project_123",
            "repository_id": "repository_123",
            "requested_by": "actor_123",
            "kind": "index",
            "state": "queued",
            "revision": 1,
            "attempt": 0,
            "repository_revision": 7,
            "source_generation": 3,
            "pinned_commit": null,
            "snapshot_id": null,
            "error_code": null,
            "created_at": "2026-11-18T00:00:00Z",
            "updated_at": "2026-11-18T00:00:00Z",
            "finished_at": null
        })
    );
    assert_eq!(
        serde_json::from_value::<RepositoryJob>(serialized)
            .unwrap()
            .id,
        job.id
    );
}
