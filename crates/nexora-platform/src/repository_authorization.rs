use serde::{Deserialize, Serialize};
/// An inscription is not a provider authorization or an engine binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepositoryAuthorization {
    AwaitingAuthorization,
    Authorized,
    Revoked,
}
