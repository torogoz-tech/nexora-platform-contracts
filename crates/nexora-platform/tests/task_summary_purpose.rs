use nexora_platform::TaskSummary;
use serde_json::json;

#[test]
fn task_summaries_preserve_execution_purpose_and_entry_workflow() {
    for purpose in ["general", "implementation", "product_design"] {
        let value = json!({
            "id":"task", "lifecycle":"pending", "project_id":null, "title":"Work",
            "type_code":"custom", "priority":"medium", "assignee_id":null,
            "backlog_id":null, "workflow_version_id":null, "state_id":null,
            "blocked":false, "archived":false, "position":null, "revision":1,
            "purpose":purpose, "entry_workflow_id":"entry-workflow"
        });
        let task: TaskSummary = serde_json::from_value(value).unwrap();
        let serialized = serde_json::to_value(task).unwrap();
        assert_eq!(serialized["purpose"], purpose);
        assert_eq!(serialized["entry_workflow_id"], "entry-workflow");
    }
}
