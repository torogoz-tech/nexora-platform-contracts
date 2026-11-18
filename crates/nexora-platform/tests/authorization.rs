use nexora_platform::{ActorKind, AuthContext, AuthMethod, Capability, CapabilityGrant, KeyScope};

#[test]
fn credentials_limit_actor_permissions_without_business_roles() {
    let grants = vec![
        CapabilityGrant::new(Capability::ProjectsRead, None),
        CapabilityGrant::new(Capability::TasksTransition, Some("project-a".into())),
    ];
    let scope = KeyScope {
        capabilities: vec![Capability::ProjectsRead, Capability::TasksTransition],
        project_ids: vec!["project-a".into()],
        all_projects: false,
        installation: false,
    };
    scope.validate().unwrap();
    for kind in [ActorKind::Human, ActorKind::Agent] {
        let context = AuthContext::new(
            "actor-1".into(),
            kind,
            AuthMethod::ApiKey,
            "credential-1".into(),
            grants.clone(),
            Some(scope.clone()),
        );
        assert!(
            context
                .require(Capability::ProjectsRead, Some("project-a"))
                .is_ok()
        );
        assert!(
            context
                .require(Capability::TasksTransition, Some("project-a"))
                .is_ok()
        );
        assert!(
            context
                .require(Capability::ProjectsRead, Some("project-b"))
                .is_err()
        );
        assert!(context.require(Capability::ActorsManage, None).is_err());
    }
}
#[test]
fn human_approval_requires_a_human_session_not_an_agent_claim_or_key() {
    let grants = vec![CapabilityGrant::new(
        Capability::ApprovalsGrant,
        Some("project-a".into()),
    )];
    let human = AuthContext::new(
        "human".into(),
        ActorKind::Human,
        AuthMethod::Session,
        "session".into(),
        grants.clone(),
        None,
    );
    assert!(human.require_human_approval("project-a").is_ok());
    let agent = AuthContext::new(
        "agent".into(),
        ActorKind::Agent,
        AuthMethod::Session,
        "session".into(),
        grants.clone(),
        None,
    );
    assert!(agent.require_human_approval("project-a").is_err());
    let key = AuthContext::new(
        "human".into(),
        ActorKind::Human,
        AuthMethod::ApiKey,
        "key".into(),
        grants,
        None,
    );
    assert!(key.require_human_approval("project-a").is_err());
}
#[test]
fn key_scopes_reject_empty_ambiguous_duplicate_and_unknown_permissions() {
    let mut scope = KeyScope {
        capabilities: vec![],
        project_ids: vec![],
        all_projects: false,
        installation: false,
    };
    assert!(scope.validate().is_err());
    scope.capabilities = vec![Capability::ProjectsRead];
    assert!(scope.validate().is_err());
    scope.all_projects = true;
    scope.project_ids = vec!["project-a".into()];
    assert!(scope.validate().is_err());
    assert!(serde_json::from_str::<Capability>("\"invented:permission\"").is_err());
}
