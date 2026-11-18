use serde::Serialize;
/// Correlation proof is delivered only as a browser cookie, never in the JSON response.
#[derive(Serialize)]
pub struct OidcAuthorization {
    pub authorization_url: String,
    pub state_id: String,
    #[serde(skip_serializing)]
    pub correlation_secret: String,
    #[serde(skip_serializing)]
    pub secure_cookie: bool,
}
