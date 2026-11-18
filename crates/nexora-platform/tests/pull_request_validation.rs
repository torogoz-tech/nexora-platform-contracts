use nexora_platform::*;
#[test]
fn rejects_unbounded_pages_and_invalid_pull_request_numbers() {
    let mut q = PullRequestQuery {
        operation: PullRequestOperation::Detail,
        number: Some(1),
        state: "all".into(),
        page: 1,
        limit: 20,
    };
    assert!(q.validate().is_ok());
    for n in [0, u64::MAX] {
        q.number = Some(n);
        assert!(q.validate().is_err());
    }
    q.number = Some(1);
    q.page = 10001;
    assert!(q.validate().is_err());
    q.page = 1;
    q.limit = 101;
    assert!(q.validate().is_err());
    q.limit = 20;
    q.state = "merged;DROP TABLE".into();
    assert!(q.validate().is_err());
}
#[test]
fn requirement_links_pin_a_version_and_task_links_cannot_have_one() {
    let mut v = PullRequestLinkCreate {
        kind: "requirement".into(),
        target_id: "doc_one".into(),
        version_id: None,
        target_revision: 1,
        reason: "Explicit relation".into(),
    };
    assert!(v.validate().is_err());
    v.version_id = Some("ver_one".into());
    assert!(v.validate().is_ok());
    v.kind = "task".into();
    assert!(v.validate().is_err());
    v.version_id = None;
    assert!(v.validate().is_ok());
    v.target_revision = 0;
    assert!(v.validate().is_err());
    v.target_revision = 1;
    v.reason = "  ".into();
    assert!(v.validate().is_err());
}
