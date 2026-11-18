use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyntaxRole {
    #[default]
    Declaration,
    Reference,
    Container,
    Expression,
}
