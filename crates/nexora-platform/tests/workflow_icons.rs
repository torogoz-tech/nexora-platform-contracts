use nexora_platform::{
    DeveloperWorkflowCreate, DeveloperWorkflowEdit, WorkflowCreate, WorkflowIcon, WorkflowSummary,
};
use serde_json::json;
#[test]
fn workflow_icons_have_a_closed_lucide_allowlist_and_backward_compatible_defaults() {
    let icons = [
        WorkflowIcon::Workflow,
        WorkflowIcon::GitBranch,
        WorkflowIcon::Code,
        WorkflowIcon::Bug,
        WorkflowIcon::ShieldCheck,
        WorkflowIcon::Rocket,
        WorkflowIcon::Layers,
        WorkflowIcon::ListTodo,
        WorkflowIcon::Briefcase,
        WorkflowIcon::Lightbulb,
        WorkflowIcon::FlaskConical,
        WorkflowIcon::GitPullRequest,
        WorkflowIcon::Palette,
        WorkflowIcon::Infinity,
        WorkflowIcon::ClipboardCheck,
        WorkflowIcon::AppWindow,
        WorkflowIcon::Server,
        WorkflowIcon::Smartphone,
        WorkflowIcon::Database,
        WorkflowIcon::Compass,
        WorkflowIcon::Headset,
        WorkflowIcon::SwatchBook,
        WorkflowIcon::PenTool,
        WorkflowIcon::MousePointer,
        WorkflowIcon::Accessibility,
        WorkflowIcon::Monitor,
        WorkflowIcon::Component,
        WorkflowIcon::Terminal,
        WorkflowIcon::Cloud,
        WorkflowIcon::Container,
        WorkflowIcon::Network,
        WorkflowIcon::Cpu,
        WorkflowIcon::Blocks,
        WorkflowIcon::TestTube,
        WorkflowIcon::CircleCheck,
        WorkflowIcon::Gauge,
        WorkflowIcon::ScanSearch,
        WorkflowIcon::LockKeyhole,
        WorkflowIcon::Target,
        WorkflowIcon::Presentation,
        WorkflowIcon::CalendarDays,
        WorkflowIcon::Users,
        WorkflowIcon::Handshake,
        WorkflowIcon::Megaphone,
        WorkflowIcon::ShoppingCart,
        WorkflowIcon::GraduationCap,
        WorkflowIcon::HeartPulse,
        WorkflowIcon::Banknote,
    ];
    for icon in icons {
        let value = json!(icon.as_str());
        assert_eq!(serde_json::to_value(icon).unwrap(), value);
        assert_eq!(serde_json::from_value::<WorkflowIcon>(value).unwrap(), icon)
    }
    for invalid in [
        json!("not-a-lucide-icon"),
        json!("GitBranch"),
        json!("git_branch"),
        json!("<svg>"),
        json!(null),
        json!(42),
    ] {
        assert!(serde_json::from_value::<WorkflowIcon>(invalid).is_err())
    }
    assert_eq!(WorkflowIcon::default(), WorkflowIcon::Workflow);
    let input = json!({"name":"Delivery","graph":{"states":[{"id":"ready","name":"Ready","initial":true}],"transitions":[]}});
    let developer: DeveloperWorkflowCreate = serde_json::from_value(input.clone()).unwrap();
    assert_eq!(developer.icon, WorkflowIcon::Workflow);
    let mut legacy = input.clone();
    legacy["resource_kinds"] = json!(["task"]);
    assert_eq!(
        serde_json::from_value::<WorkflowCreate>(legacy)
            .unwrap()
            .icon,
        WorkflowIcon::Workflow
    );
    let edit: DeveloperWorkflowEdit =
        serde_json::from_value(json!({"expected_revision":1,"reason":"Metadata"})).unwrap();
    assert_eq!(edit.icon, None);
    let mut invalid = input;
    invalid["icon"] = json!("not-allowed");
    assert!(serde_json::from_value::<DeveloperWorkflowCreate>(invalid).is_err());
    let summary:WorkflowSummary=serde_json::from_value(json!({"id":"w","project_id":null,"name":"Legacy","purpose":"","resource_kinds":["task"],"archived":false,"revision":1,"latest_version_id":null,"latest_version_number":0})).unwrap();
    assert_eq!(summary.icon, WorkflowIcon::Workflow);
}

#[test]
fn workflow_input_serialization_preserves_pre_icon_request_fingerprints() {
    let graph = json!({"states":[{"id":"ready","name":"Ready","initial":true,"terminal":false}],"transitions":[]});
    let original_create = json!({"name":"Legacy","purpose":"","graph":graph});
    let developer: DeveloperWorkflowCreate =
        serde_json::from_value(original_create.clone()).unwrap();
    assert_eq!(serde_json::to_value(&developer).unwrap(), original_create);
    let mut explicit_default = original_create.clone();
    explicit_default["icon"] = json!("workflow");
    let defaulted: DeveloperWorkflowCreate = serde_json::from_value(explicit_default).unwrap();
    assert_eq!(serde_json::to_value(defaulted).unwrap(), original_create);
    let mut nondefault = developer;
    nondefault.icon = WorkflowIcon::Rocket;
    assert_eq!(serde_json::to_value(nondefault).unwrap()["icon"], "rocket");
    let mut legacy_create = original_create;
    legacy_create["resource_kinds"] = json!(["task"]);
    let legacy: WorkflowCreate = serde_json::from_value(legacy_create.clone()).unwrap();
    assert_eq!(serde_json::to_value(legacy).unwrap(), legacy_create);
    let old_change = json!({"expected_revision":1,"name":null,"purpose":null,"graph":null,"archived":null,"reason":"Metadata"});
    let edit: DeveloperWorkflowEdit = serde_json::from_value(old_change.clone()).unwrap();
    assert_eq!(serde_json::to_value(&edit).unwrap(), old_change);
    let mut explicit_default = edit;
    explicit_default.icon = Some(WorkflowIcon::Workflow);
    assert_eq!(
        serde_json::to_value(explicit_default).unwrap()["icon"],
        "workflow"
    );
    let mut old_legacy_change = old_change;
    old_legacy_change["resource_kinds"] = json!(null);
    let legacy: nexora_platform::WorkflowChange =
        serde_json::from_value(old_legacy_change.clone()).unwrap();
    assert_eq!(serde_json::to_value(legacy).unwrap(), old_legacy_change);
    let response:WorkflowSummary=serde_json::from_value(json!({"id":"w","project_id":null,"name":"Legacy","purpose":"","resource_kinds":["task"],"archived":false,"revision":1,"latest_version_id":null,"latest_version_number":0})).unwrap();
    assert_eq!(serde_json::to_value(response).unwrap()["icon"], "workflow");
}
