use crate::ApiKeyMetadata;
use serde::Serialize;
/// Secret appears only in the issuance response; deliberately no Debug/Deserialize.
#[derive(Serialize)]
pub struct IssuedApiKey {
    pub key: String,
    pub metadata: ApiKeyMetadata,
}
