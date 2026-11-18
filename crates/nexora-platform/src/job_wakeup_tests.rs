use crate::{JobKind, JobWakeup};
use serde_json::json;

#[test]
fn wakeup_contains_only_versioned_job_identity_and_kind() {
    let wakeup = JobWakeup::new("job_123".into(), JobKind::RepositoryIndex).unwrap();

    let value = serde_json::to_value(wakeup).unwrap();

    assert_eq!(
        value,
        json!({
            "version": 1,
            "job_id": "job_123",
            "kind": "repository_index"
        })
    );
}

#[test]
fn wakeup_rejects_empty_or_unbounded_job_ids() {
    assert!(JobWakeup::new(String::new(), JobKind::RepositoryIndex).is_err());
    assert!(JobWakeup::new("x".repeat(129), JobKind::RepositoryIndex).is_err());
    assert!(JobWakeup::new("job_\n123".into(), JobKind::RepositoryIndex).is_err());
}

#[test]
fn wakeup_rejects_unknown_fields_and_contract_versions() {
    let unknown_field = json!({
        "version": 1,
        "job_id": "job_123",
        "kind": "repository_index",
        "payload": "must not be transported"
    });
    let unsupported_version = json!({
        "version": 2,
        "job_id": "job_123",
        "kind": "repository_index"
    });

    assert!(serde_json::from_value::<JobWakeup>(unknown_field).is_err());
    assert!(serde_json::from_value::<JobWakeup>(unsupported_version).is_err());
}

#[test]
fn job_kinds_route_to_their_own_worker_queue() {
    assert_eq!(
        JobKind::RepositorySync.routing_key(),
        "nexora.jobs.analysis"
    );
    assert_eq!(
        JobKind::RepositoryIndex.routing_key(),
        "nexora.jobs.analysis"
    );
    assert_eq!(
        JobKind::ReleaseComposition.routing_key(),
        "nexora.jobs.release"
    );
    assert_eq!(
        JobKind::TaskTransitionResume.routing_key(),
        "nexora.jobs.release"
    );
    assert_eq!(
        JobKind::KnowledgeSynthesis.routing_key(),
        "nexora.jobs.knowledge"
    );
    assert_eq!(
        JobKind::KnowledgeProcessing.routing_key(),
        "nexora.jobs.knowledge"
    );
    assert_eq!(
        JobKind::DailyDeveloperAnalysis.routing_key(),
        "nexora.jobs.knowledge"
    );
    assert_eq!(JobKind::TaskReview.routing_key(), "nexora.jobs.knowledge");
    assert_eq!(
        JobKind::EmailNotification.routing_key(),
        "nexora.jobs.notification"
    );
}
