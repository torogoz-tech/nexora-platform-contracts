use nexora_platform::{JobKind, JobWakeup};
use serde_json::json;

#[test]
fn job_wakeup_preserves_versioned_wire_shape_and_routing_names() {
    let wakeup = JobWakeup::new("job_123".into(), JobKind::RepositoryIndex).unwrap();

    assert_eq!(
        serde_json::to_value(&wakeup).unwrap(),
        json!({
            "version": 1,
            "job_id": "job_123",
            "kind": "repository_index"
        })
    );
    assert_eq!(
        serde_json::from_value::<JobWakeup>(
            json!({"version":1,"job_id":"job_123","kind":"repository_index"})
        )
        .unwrap(),
        wakeup
    );

    let kinds = [
        (
            JobKind::RepositorySync,
            "repository_sync",
            "nexora.jobs.analysis",
        ),
        (
            JobKind::RepositoryIndex,
            "repository_index",
            "nexora.jobs.analysis",
        ),
        (
            JobKind::TaskTransitionResume,
            "task_transition_resume",
            "nexora.jobs.release",
        ),
        (
            JobKind::ReleaseComposition,
            "release_composition",
            "nexora.jobs.release",
        ),
        (
            JobKind::KnowledgeSynthesis,
            "knowledge_synthesis",
            "nexora.jobs.knowledge",
        ),
        (
            JobKind::KnowledgeProcessing,
            "knowledge_processing",
            "nexora.jobs.knowledge",
        ),
        (
            JobKind::DailyDeveloperAnalysis,
            "daily_developer_analysis",
            "nexora.jobs.knowledge",
        ),
        (JobKind::TaskReview, "task_review", "nexora.jobs.knowledge"),
        (
            JobKind::EmailNotification,
            "email_notification",
            "nexora.jobs.notification",
        ),
    ];

    for (kind, wire_name, routing_key) in kinds {
        assert_eq!(serde_json::to_value(kind).unwrap(), json!(wire_name));
        assert_eq!(kind.routing_key(), routing_key);
        assert_eq!(kind.queue_name(), routing_key);
    }
}

#[test]
fn job_wakeup_rejects_unknown_versions_and_unsafe_identifiers() {
    assert!(
        serde_json::from_value::<JobWakeup>(
            json!({"version":2,"job_id":"job_123","kind":"repository_index"})
        )
        .is_err()
    );
    assert!(JobWakeup::new("job/123".into(), JobKind::RepositoryIndex).is_err());
}
