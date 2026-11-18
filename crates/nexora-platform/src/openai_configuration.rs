use crate::{OpenAiEndpoint, PlatformError};
use serde::{Deserialize, Serialize};
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OpenAiConfiguration {
    pub endpoint: String,
    pub model: String,
    pub embedding_model: String,
    pub effort: String,
    pub enabled: bool,
}
impl OpenAiConfiguration {
    pub fn validate(&self) -> Result<(), PlatformError> {
        OpenAiEndpoint::normalize(&self.endpoint)?;
        if self.model.len() > 128
            || !self
                .model
                .bytes()
                .all(|c| c.is_ascii_alphanumeric() || b"-._:/".contains(&c))
            || self.embedding_model.len() > 128
            || !self
                .embedding_model
                .bytes()
                .all(|c| c.is_ascii_alphanumeric() || b"-._:/".contains(&c))
            || !matches!(self.effort.as_str(), "low" | "medium" | "high")
            || self.enabled && (self.model.is_empty() || self.embedding_model.is_empty())
        {
            return Err(PlatformError::Validation(
                "invalid OpenAI model or reasoning effort",
            ));
        }
        Ok(())
    }
}
