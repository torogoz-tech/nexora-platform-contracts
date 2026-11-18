use crate::PlatformError;
use serde::{Deserialize, Serialize};

/// One installation-owned envelope. Values are substituted once, never interpreted as templates.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmailTemplate {
    pub html: String,
    pub revision: i64,
}
impl EmailTemplate {
    pub const DEFAULT_HTML: &str = include_str!("email_template.html");
    pub fn validate(html: &str) -> Result<(), PlatformError> {
        if html.len() > 100_000 || html.matches("{{body}}").count() != 1 {
            return Err(PlatformError::Validation(
                "email template must contain exactly one {{body}} placeholder and at most 100000 bytes",
            ));
        }
        let mut remaining = html;
        while let Some(start) = remaining.find("{{") {
            remaining = &remaining[start..];
            let Some(end) = remaining.find("}}") else {
                return Err(PlatformError::Validation(
                    "invalid email template placeholder",
                ));
            };
            if !matches!(
                &remaining[..end + 2],
                "{{body}}" | "{{subject}}" | "{{organization_name}}"
            ) {
                return Err(PlatformError::Validation(
                    "unknown email template placeholder",
                ));
            }
            remaining = &remaining[end + 2..];
        }
        Ok(())
    }
    fn escape(value: &str) -> String {
        value
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#39;")
    }
    pub fn render(&self, organization: &str, subject: &str, body: &str) -> String {
        self.render_html(
            organization,
            subject,
            &Self::escape(body).replace('\n', "<br>"),
        )
    }
    /// Body HTML is compiled from admin-owned content with escaped dynamic values.
    pub fn render_html(&self, organization: &str, subject: &str, body_html: &str) -> String {
        let mut result = String::new();
        let mut remaining = self.html.as_str();
        while let Some(start) = remaining.find("{{") {
            result.push_str(&remaining[..start]);
            remaining = &remaining[start..];
            let Some(end) = remaining.find("}}") else {
                break;
            };
            match &remaining[..end + 2] {
                "{{organization_name}}" => result.push_str(&Self::escape(organization)),
                "{{subject}}" => result.push_str(&Self::escape(subject)),
                "{{body}}" => result.push_str(body_html),
                other => result.push_str(other),
            }
            remaining = &remaining[end + 2..];
        }
        result.push_str(remaining);
        result
    }
}
