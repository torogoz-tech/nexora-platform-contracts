use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskDependencyKind {
    Blocks,
    DependsOn,
    RelatedTo,
}
impl TaskDependencyKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Blocks => "blocks",
            Self::DependsOn => "depends_on",
            Self::RelatedTo => "related_to",
        }
    }
}
