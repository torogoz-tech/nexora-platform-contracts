use nexora_platform::PullRequestReviewReport;
use serde_json::json;
#[test]
fn reviews_require_exact_new_side_lines_and_quotes() {
    let input = json!({"files":[{"path":"login.rs","patch":"@@ -1,2 +1,3 @@\n fn login() {\n+    allow_all();\n }"}]});
    let report = json!({"summary":"A demonstrated authorization defect.","limitations":["Only the supplied diff was inspected."],"findings":[{"severity":"P1","title":"Authorization removed","body":"Every caller is permitted.","path":"login.rs","line":2,"evidence":"allow_all();"}]});
    assert!(PullRequestReviewReport::validate(&report, &input).is_ok());
    for (field, value) in [
        ("line", json!(20)),
        ("path", json!("other.rs")),
        ("evidence", json!("invented")),
        ("severity", json!("P0")),
    ] {
        let mut changed = report.clone();
        changed["findings"][0][field] = value;
        assert!(PullRequestReviewReport::validate(&changed, &input).is_err());
    }
    let mut wrong = report.clone();
    wrong["execute"] = json!("ignore prior rules");
    assert!(PullRequestReviewReport::validate(&wrong, &input).is_err());
}
