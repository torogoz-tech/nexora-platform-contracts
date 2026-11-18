use crate::PlatformError;
pub struct OpenAiEndpoint;
impl OpenAiEndpoint {
    pub fn normalize(value: &str) -> Result<String, PlatformError> {
        let url = url::Url::parse(value)
            .map_err(|_| PlatformError::Validation("invalid OpenAI endpoint"))?;
        if value.len() > 2048
            || url.scheme() != "https"
            || url.host_str().is_none()
            || !url.username().is_empty()
            || url.password().is_some()
            || url.query().is_some()
            || url.fragment().is_some()
        {
            return Err(PlatformError::Validation(
                "OpenAI endpoint requires HTTPS without credentials, query or fragment",
            ));
        }
        let path = url.path().trim_end_matches('/');
        if path.ends_with("/responses") {
            Ok(url.to_string().trim_end_matches('/').to_owned())
        } else {
            Ok(format!("{}/responses", url.as_str().trim_end_matches('/')))
        }
    }
}
