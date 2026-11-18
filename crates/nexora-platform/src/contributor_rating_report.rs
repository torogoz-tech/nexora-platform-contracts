use crate::{PlatformError, TaskWorkBranchReport};
use serde_json::Value;
/// Private rating assertions must cite changed lines in this person's verified commits.
pub struct ContributorRatingReport;
impl ContributorRatingReport {
    pub fn score(report: &Value, input: &Value) -> Result<Option<f64>, PlatformError> {
        let invalid = || PlatformError::Validation("invalid private contributor rating evidence");
        if report.as_object().is_none_or(|v| v.len() != 4) || !Self::text(&report["reason"], 4000) {
            return Err(invalid());
        }
        if serde_json::to_vec(report).map_err(|_| invalid())?.len() > 65536 {
            return Err(invalid());
        }
        let limitations = report["limitations"]
            .as_array()
            .filter(|v| v.len() <= 16)
            .ok_or_else(invalid)?;
        if limitations.iter().any(|v| !Self::text(v, 2000)) {
            return Err(invalid());
        }
        let rubric = report["rubric"].as_array().ok_or_else(invalid)?;
        if report["status"] == "insufficient_evidence" {
            if !rubric.is_empty() {
                return Err(invalid());
            }
            return Ok(None);
        }
        if report["status"] != "rated" {
            return Err(invalid());
        }
        let mut expected = vec!["correctness", "security", "maintainability"];
        if input["criteria"].as_array().is_some_and(|v| !v.is_empty()) {
            expected.push("criteria")
        }
        if rubric.len() != expected.len() {
            return Err(invalid());
        }
        let mut seen = std::collections::BTreeSet::new();
        let mut total = 0.;
        for item in rubric {
            let dimension = item["dimension"].as_str().ok_or_else(invalid)?;
            if item.as_object().is_none_or(|v| v.len() != 4)
                || !expected.contains(&dimension)
                || !seen.insert(dimension)
                || !Self::text(&item["reason"], 2000)
            {
                return Err(invalid());
            }
            let score = item["score"]
                .as_f64()
                .filter(|v| v.is_finite() && (0.0..=5.0).contains(v))
                .ok_or_else(invalid)?;
            total += score;
            let evidence = item["evidence"]
                .as_array()
                .filter(|v| !v.is_empty() && v.len() <= 4)
                .ok_or_else(invalid)?;
            for quote in evidence {
                Self::evidence(quote, input)?;
            }
        }
        if serde_json::to_vec(report).map_err(|_| invalid())?.len() > 65536 {
            return Err(invalid());
        }
        Ok(Some(total / (expected.len() as f64)))
    }
    fn evidence(value: &Value, input: &Value) -> Result<(), PlatformError> {
        let invalid = || PlatformError::Validation("invalid private contributor rating evidence");
        if value.as_object().is_none_or(|v| v.len() != 5) || !Self::text(&value["quote"], 2000) {
            return Err(invalid());
        }
        let sha = value["commit_sha"]
            .as_str()
            .filter(|v| TaskWorkBranchReport::valid_sha(v))
            .ok_or_else(invalid)?;
        let commit = input["commits"]
            .as_array()
            .and_then(|v| v.iter().find(|c| c["sha"] == sha))
            .ok_or_else(invalid)?;
        let file = commit["files"]
            .as_array()
            .and_then(|v| v.iter().find(|f| f["path"] == value["path"]))
            .ok_or_else(invalid)?;
        let side = value["side"]
            .as_str()
            .filter(|v| matches!(*v, "old" | "new"))
            .ok_or_else(invalid)?;
        let wanted = value["line"]
            .as_u64()
            .filter(|n| *n > 0)
            .ok_or_else(invalid)?;
        let quote = value["quote"].as_str().ok_or_else(invalid)?;
        let mut old = 0u64;
        let mut new = 0u64;
        let mut found = false;
        for line in file["patch"].as_str().ok_or_else(invalid)?.lines() {
            if line.starts_with("@@ ") {
                let mut parts = line.split_whitespace();
                parts.next();
                old = parts
                    .next()
                    .and_then(|s| s.strip_prefix('-'))
                    .and_then(|s| s.split(',').next())
                    .and_then(|s| s.parse().ok())
                    .ok_or_else(invalid)?;
                new = parts
                    .next()
                    .and_then(|s| s.strip_prefix('+'))
                    .and_then(|s| s.split(',').next())
                    .and_then(|s| s.parse().ok())
                    .ok_or_else(invalid)?;
            } else if let Some(text) = line.strip_prefix('+') {
                if side == "new" && new == wanted && text == quote {
                    found = true
                }
                new += 1;
            } else if let Some(text) = line.strip_prefix('-') {
                if side == "old" && old == wanted && text == quote {
                    found = true
                }
                old += 1;
            } else if line.starts_with(' ') {
                old += 1;
                new += 1;
            }
        }
        if !found {
            return Err(invalid());
        }
        Ok(())
    }
    fn text(v: &Value, max: usize) -> bool {
        v.as_str()
            .is_some_and(|s| !s.trim().is_empty() && s.len() <= max)
    }
}
#[cfg(test)]
#[path = "contributor_rating_report_tests.rs"]
mod tests;
