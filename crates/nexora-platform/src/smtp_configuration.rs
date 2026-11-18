use crate::PlatformError;
use serde::{Deserialize, Serialize};
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SmtpConfiguration {
    pub enabled: bool,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub sender_email: String,
    pub sender_name: String,
    pub tls_mode: String,
}
impl SmtpConfiguration {
    pub fn validate(&self) -> Result<(), PlatformError> {
        if self.port == 0
            || self.host.len() > 253
            || !self
                .host
                .bytes()
                .all(|c| c.is_ascii_alphanumeric() || b".-:".contains(&c))
            || self.username.len() > 256
            || self.sender_name.len() > 160
            || self.sender_email.len() > 320
            || [&self.username, &self.sender_name, &self.sender_email]
                .iter()
                .any(|v| v.chars().any(char::is_control))
            || !matches!(self.tls_mode.as_str(), "starttls" | "tls")
            || self.enabled
                && (self.host.is_empty()
                    || !email_address::EmailAddress::is_valid(&self.sender_email))
        {
            return Err(PlatformError::Validation("invalid SMTP configuration"));
        }
        Ok(())
    }
}
