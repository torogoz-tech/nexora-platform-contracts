use crate::PlatformError;
pub struct DomainText;
impl DomainText {
    pub fn required(value: &str, max: usize) -> Result<String, PlatformError> {
        let value = value.trim();
        if value.is_empty()
            || value.len() > max
            || value
                .chars()
                .any(|c| c.is_control() && !matches!(c, '\n' | '\t'))
        {
            return Err(PlatformError::Validation(
                "text empty, oversized or contains rejected characters",
            ));
        }
        Ok(value.into())
    }
    pub fn optional(value: &str, max: usize) -> Result<String, PlatformError> {
        if value.trim().is_empty() {
            Ok(String::new())
        } else {
            Self::required(value, max)
        }
    }
    pub fn name(value: &str) -> Result<String, PlatformError> {
        let value = Self::required(value, 160)?;
        if value.chars().any(char::is_control) {
            return Err(PlatformError::Validation(
                "name contains control characters",
            ));
        }
        Ok(value)
    }
    pub fn username(value: &str) -> Result<String, PlatformError> {
        let value = value.trim().to_ascii_lowercase();
        if value.is_empty()
            || value.len() > 254
            || !value
                .bytes()
                .all(|c| c.is_ascii_alphanumeric() || b"@._+-".contains(&c))
        {
            return Err(PlatformError::Validation("invalid username"));
        }
        Ok(value)
    }
}
