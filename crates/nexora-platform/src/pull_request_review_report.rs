use crate::PlatformError;
use serde_json::Value;
/// Validates model claims against the exact supplied new-side diff lines.
pub struct PullRequestReviewReport;
impl PullRequestReviewReport {
    pub fn validate(report: &Value, input: &Value) -> Result<(), PlatformError> {
        let invalid = || PlatformError::Validation("invalid pull request review evidence");
        let object = report.as_object().ok_or_else(invalid)?;
        if object.len() != 3 || !Self::text(&report["summary"], 8000) {
            return Err(invalid());
        }
        let limitations = report["limitations"]
            .as_array()
            .filter(|a| a.len() <= 16)
            .ok_or_else(invalid)?;
        if limitations.iter().any(|v| !Self::text(v, 2000)) {
            return Err(invalid());
        }
        let findings = report["findings"]
            .as_array()
            .filter(|a| a.len() <= 16)
            .ok_or_else(invalid)?;
        let files = input["files"].as_array().ok_or_else(invalid)?;
        for finding in findings {
            if finding.as_object().is_none_or(|v| v.len() != 6)
                || !["P1", "P2", "P3"].contains(&finding["severity"].as_str().unwrap_or(""))
                || !Self::text(&finding["title"], 200)
                || !Self::text(&finding["body"], 4000)
                || !Self::text(&finding["evidence"], 2000)
            {
                return Err(invalid());
            }
            let file = files
                .iter()
                .find(|f| f["path"] == finding["path"])
                .ok_or_else(invalid)?;
            let wanted = finding["line"]
                .as_u64()
                .filter(|v| *v > 0)
                .ok_or_else(invalid)?;
            let quote = finding["evidence"].as_str().ok_or_else(invalid)?;
            let patch = file["patch"].as_str().ok_or_else(invalid)?;
            let mut line = 0;
            let mut found = false;
            for text in patch.lines() {
                if text.starts_with("@@ ") {
                    line = text
                        .split_whitespace()
                        .nth(2)
                        .and_then(|v| v.strip_prefix('+'))
                        .and_then(|v| v.split(',').next())
                        .and_then(|v| v.parse::<u64>().ok())
                        .ok_or_else(invalid)?;
                } else if (text.starts_with('+') || text.starts_with(' ')) && line > 0 {
                    if line == wanted && text[1..].contains(quote) {
                        found = true;
                    }
                    line += 1;
                }
            }
            if !found {
                return Err(invalid());
            }
        }
        if serde_json::to_vec(report).map_err(|_| invalid())?.len() > 131072 {
            return Err(invalid());
        }
        Ok(())
    }
    fn text(value: &Value, max: usize) -> bool {
        value
            .as_str()
            .is_some_and(|s| !s.trim().is_empty() && s.len() <= max)
    }
}
