use serde::Serialize;
/// Deliberately no Debug/Deserialize for the bearer credential.
#[derive(Serialize)]
pub struct IssuedSession {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub actor_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_step: Option<String>,
}
