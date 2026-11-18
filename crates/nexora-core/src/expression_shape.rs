use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressionShape {
    pub kind: crate::ExpressionKind,
    pub operator: Option<String>,
    pub operands: Vec<crate::NodeId>,
}
