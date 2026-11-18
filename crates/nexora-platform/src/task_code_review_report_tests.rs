use super::TaskCodeReviewReport;
use serde_json::json;
#[test]
fn criterion_coverage_and_evidence_are_validated_separately_from_execution() {
    let input = json!({"files":[{"path":"source.rs","patch":"@@ -1 +1 @@\n-old\n+new"}],"criteria":[{"id":"C1","description":"Correctness"},{"id":"C2","description":"Performance"}]});
    let mut report = json!({"summary":"Static review only.","findings":[],"limitations":["No tests were executed."],"criteria_results":[{"id":"C1","outcome":"supported","evidence":"new code"},{"id":"C2","outcome":"unverified","evidence":"Runtime measurement required"}]});
    TaskCodeReviewReport::validate(&report, &input).unwrap();
    report["criteria_results"][1]["id"] = json!("C1");
    assert!(TaskCodeReviewReport::validate(&report, &input).is_err());
    report["criteria_results"][1]["id"] = json!("C2");
    report["criteria_results"][1]["outcome"] = json!("tests_passed");
    assert!(TaskCodeReviewReport::validate(&report, &input).is_err());
}
#[test]
fn pre_pr_findings_cannot_claim_lines_outside_pinned_diff() {
    let input =
        json!({"files":[{"path":"source.rs","patch":"@@ -1 +1 @@\n-old\n+new"}],"criteria":[]});
    let report = json!({"summary":"Static review only.","findings":[{"severity":"P1","title":"Defect","body":"Why","path":"source.rs","line":8,"evidence":"invented"}],"limitations":[],"criteria_results":[]});
    assert!(TaskCodeReviewReport::validate(&report, &input).is_err());
}
