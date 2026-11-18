use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InstallationService {
    Openai,
    Smtp,
}
impl InstallationService {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Openai => "openai",
            Self::Smtp => "smtp",
        }
    }
}
