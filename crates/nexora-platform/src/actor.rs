use crate::{ActorKind, DomainText, PlatformError};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actor {
    pub id: String,
    pub kind: ActorKind,
    pub name: String,
    pub purpose: String,
    pub active: bool,
    pub revision: i64,
    pub created_at: String,
    pub updated_at: String,
}
impl Actor {
    pub fn new(
        id: String,
        kind: ActorKind,
        name: &str,
        purpose: &str,
    ) -> Result<Self, PlatformError> {
        Ok(Self {
            id,
            kind,
            name: DomainText::name(name)?,
            purpose: DomainText::required(purpose, 2000)?,
            active: true,
            revision: 1,
            created_at: String::new(),
            updated_at: String::new(),
        })
    }
}
