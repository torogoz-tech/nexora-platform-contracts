use nexora_platform::*;
#[test]
fn roles_grant_global_configuration_to_managers_and_keep_approvals_out_of_developers() {
    for role in [
        MemberRole::Developer,
        MemberRole::TechnicalLead,
        MemberRole::Pmo,
        MemberRole::Manager,
    ] {
        let caps = role.capabilities();
        assert!(caps.contains(&Capability::TasksTransition));
        assert_eq!(
            caps.contains(&Capability::InstallationManage),
            role == MemberRole::Manager
        );
        assert_eq!(
            caps.contains(&Capability::ActorsManage),
            role == MemberRole::Manager
        );
        assert_eq!(
            caps.contains(&Capability::ApprovalsGrant),
            role != MemberRole::Developer
        );
    }
    assert_eq!(MemberRole::SuperAdmin.capabilities(), Capability::all());
}
#[test]
fn role_change_rejects_super_admin_ambiguous_and_duplicate_scopes() {
    let mut value = MemberRoleChange {
        role: MemberRole::Developer,
        all_projects: false,
        project_ids: vec!["p".into()],
        expected_revision: 0,
    };
    assert!(value.validate().is_ok());
    value.project_ids.push("p".into());
    assert!(value.validate().is_err());
    value.project_ids.pop();
    value.all_projects = true;
    assert!(value.validate().is_err());
    value.project_ids.clear();
    assert!(value.validate().is_ok());
    value.all_projects = false;
    assert!(value.validate().is_err());
    value.all_projects = true;
    value.role = MemberRole::SuperAdmin;
    assert!(value.validate().is_err());
}
