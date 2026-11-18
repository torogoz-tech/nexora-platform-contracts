use crate::{PlatformError, PullRequestReviewReport};
use serde_json::{Value, json};
pub struct TaskCodeReviewReport;
impl TaskCodeReviewReport {
    pub fn validate(report: &Value, input: &Value) -> Result<(), PlatformError> {
        let invalid = || PlatformError::Validation("invalid pre-PR task review evidence");
        if report.as_object().is_none_or(|r| r.len() != 4) {
            return Err(invalid());
        }
        PullRequestReviewReport::validate(
            &json!({"summary":report["summary"],"findings":report["findings"],"limitations":report["limitations"]}),
            input,
        )?;
        let criteria = input["criteria"].as_array().ok_or_else(invalid)?;
        let results = report["criteria_results"]
            .as_array()
            .filter(|r| r.len() == criteria.len())
            .ok_or_else(invalid)?;
        let mut seen = std::collections::BTreeSet::new();
        for result in results {
            let id = result["id"].as_str().ok_or_else(invalid)?;
            if result.as_object().is_none_or(|r| r.len() != 3)
                || !seen.insert(id)
                || !criteria.iter().any(|c| c["id"] == id)
                || !["supported", "gap", "unverified"]
                    .contains(&result["outcome"].as_str().unwrap_or(""))
                || result["evidence"]
                    .as_str()
                    .is_none_or(|s| s.trim().is_empty() || s.len() > 4000)
            {
                return Err(invalid());
            }
        }
        if serde_json::to_vec(report).map_err(|_| invalid())?.len() > 131072 {
            return Err(invalid());
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "task_code_review_report_tests.rs"]
mod tests;
