use serde::Serialize;
#[derive(Serialize)]
pub struct GitAuthorization {
    pub authorization_url: String,
    pub state_id: String,
    #[serde(skip_serializing)]
    pub correlation_secret: String,
    #[serde(skip_serializing)]
    pub secure_cookie: bool,
}
