use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DocumentLinkKind {
    References,
    Implements,
    RequiredForTransition,
}
impl DocumentLinkKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::References => "references",
            Self::Implements => "implements",
            Self::RequiredForTransition => "required_for_transition",
        }
    }
}
