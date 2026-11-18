use super::ContributorRatingReport;
use serde_json::{Value, json};
fn context() -> Value {
    json!({"criteria":[],"commits":[{"sha":"a".repeat(40),"files":[{"path":"service.rs","patch":"@@ -1,2 +1,2 @@\n-unsafe_call();\n+checked_call();\n unchanged();"}]}]})
}
fn report() -> Value {
    json!({"status":"rated","reason":"Static evidence only; author-owned changes.","limitations":["No tests were executed."],"rubric":(["correctness","security","maintainability"].map(|dimension|json!({"dimension":dimension,"score":4.,"reason":"Changed line guards the call.","evidence":[{"commit_sha":"a".repeat(40),"path":"service.rs","side":"new","line":1,"quote":"checked_call();"}]})))})
}
#[test]
fn score_is_mean_of_valid_rubric_and_owned_changed_lines() {
    assert_eq!(
        ContributorRatingReport::score(&report(), &context()).unwrap(),
        Some(4.)
    );
    let mut deleted = report();
    deleted["rubric"][0]["evidence"][0] = json!({"commit_sha":"a".repeat(40),"path":"service.rs","side":"old","line":1,"quote":"unsafe_call();"});
    assert!(ContributorRatingReport::score(&deleted, &context()).is_ok())
}
#[test]
fn fabricated_foreign_context_and_scores_are_rejected() {
    for (key, value) in [
        ("commit_sha", json!("b".repeat(40))),
        ("path", json!("other.rs")),
        ("line", json!(2)),
        ("quote", json!("unchanged();")),
        ("side", json!("context")),
    ] {
        let mut r = report();
        r["rubric"][0]["evidence"][0][key] = value;
        assert!(ContributorRatingReport::score(&r, &context()).is_err());
    }
    for score in [-1., 5.1] {
        let mut r = report();
        r["rubric"][0]["score"] = json!(score);
        assert!(ContributorRatingReport::score(&r, &context()).is_err())
    }
}
#[test]
fn incomplete_rubric_does_not_create_default_points() {
    let mut r = report();
    r["rubric"] = json!([]);
    assert!(ContributorRatingReport::score(&r, &context()).is_err());
    r["status"] = json!("insufficient_evidence");
    assert_eq!(
        ContributorRatingReport::score(&r, &context()).unwrap(),
        None
    );
    let mut input = context();
    input["criteria"] = json!([{"id":"C1"}]);
    assert!(ContributorRatingReport::score(&report(), &input).is_err())
}
