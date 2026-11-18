use crate::{EmailTemplateParameter, PlatformError};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmailContentTemplate {
    pub kind: String,
    pub language: String,
    pub subject: String,
    pub body_html: String,
    pub default_subject: String,
    pub default_body_html: String,
    pub revision: i64,
    pub parameters: Vec<EmailTemplateParameter>,
    pub customized: bool,
    pub delivery_enabled: bool,
}
impl EmailContentTemplate {
    pub fn defaults() -> Vec<Self> {
        serde_json::from_str(include_str!("email_content_defaults.json"))
            .expect("bundled email defaults")
    }
    pub fn definition(kind: &str, language: &str) -> Result<Self, PlatformError> {
        Self::defaults()
            .into_iter()
            .find(|v| v.kind == kind && v.language == language)
            .ok_or(PlatformError::NotFound)
    }
    pub fn validate(&self, subject: &str, body: &str) -> Result<(), PlatformError> {
        if subject.trim().is_empty()
            || subject.len() > 1000
            || subject.chars().any(char::is_control)
            || body.trim().is_empty()
            || body.len() > 100000
        {
            return Err(PlatformError::Validation(
                "invalid email content size or subject",
            ));
        }
        for value in [subject, body] {
            let mut remaining = value;
            while let Some(start) = remaining.find("{{") {
                remaining = &remaining[start + 2..];
                let end = remaining
                    .find("}}")
                    .ok_or(PlatformError::Validation("invalid email parameter"))?;
                if !self.parameters.iter().any(|p| p.name == remaining[..end]) {
                    return Err(PlatformError::Validation("unknown email parameter"));
                }
                remaining = &remaining[end + 2..];
            }
        }
        for p in self.parameters.iter().filter(|p| p.required) {
            let token = format!("{{{{{}}}}}", p.name);
            // Functional parameters must remain in the message body, not only its subject.
            if !body.contains(&token) {
                return Err(PlatformError::Validation(
                    "required email parameter missing",
                ));
            }
            if p.value_kind == "url" {
                let document = scraper::Html::parse_fragment(body);
                let selector =
                    scraper::Selector::parse("a[href]").expect("constant email link selector");
                if !document
                    .select(&selector)
                    .any(|a| a.value().attr("href") == Some(token.as_str()))
                {
                    return Err(PlatformError::Validation("required email link missing"));
                }
            }
        }
        Ok(())
    }
    pub fn render(
        &self,
        values: &BTreeMap<String, String>,
    ) -> Result<(String, String), PlatformError> {
        self.validate(&self.subject, &self.body_html)?;
        for p in &self.parameters {
            if let Some(value) = values.get(&p.name)
                && p.value_kind == "url"
            {
                let url = url::Url::parse(value)
                    .map_err(|_| PlatformError::Validation("invalid email URL"))?;
                if !matches!(url.scheme(), "http" | "https")
                    || url.host_str().is_none()
                    || !url.username().is_empty()
                    || url.password().is_some()
                {
                    return Err(PlatformError::Validation("invalid email URL"));
                }
            }
        }
        let subject = Self::substitute(&self.subject, values, false)?;
        if subject.chars().any(char::is_control) || subject.len() > 2000 {
            return Err(PlatformError::Validation("invalid rendered email subject"));
        }
        Ok((subject, Self::substitute(&self.body_html, values, true)?))
    }
    fn substitute(
        source: &str,
        values: &BTreeMap<String, String>,
        escape: bool,
    ) -> Result<String, PlatformError> {
        let mut result = String::new();
        let mut remaining = source;
        while let Some(start) = remaining.find("{{") {
            result.push_str(&remaining[..start]);
            remaining = &remaining[start + 2..];
            let end = remaining
                .find("}}")
                .ok_or(PlatformError::Validation("invalid email parameter"))?;
            let value = values
                .get(&remaining[..end])
                .ok_or(PlatformError::Validation("missing email parameter value"))?;
            if escape {
                result.push_str(
                    &value
                        .replace('&', "&amp;")
                        .replace('<', "&lt;")
                        .replace('>', "&gt;")
                        .replace('"', "&quot;")
                        .replace('\'', "&#39;"),
                );
            } else {
                result.push_str(value);
            }
            remaining = &remaining[end + 2..];
        }
        result.push_str(remaining);
        Ok(result)
    }
}
