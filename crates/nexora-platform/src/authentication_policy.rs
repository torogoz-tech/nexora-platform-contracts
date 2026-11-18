use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthenticationPolicy {
    pub local_enabled: bool,
    pub revision: i64,
}
