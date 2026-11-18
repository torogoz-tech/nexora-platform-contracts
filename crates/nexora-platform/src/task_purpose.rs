use serde::{Deserialize, Serialize};
/// Stable semantics independent of an editable task type code or display name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskPurpose {
    ProductDesign,
    Implementation,
    #[default]
    General,
}
impl TaskPurpose {
    pub fn is_general(&self) -> bool {
        *self == Self::General
    }
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ProductDesign => "product_design",
            Self::Implementation => "implementation",
            Self::General => "general",
        }
    }
}
