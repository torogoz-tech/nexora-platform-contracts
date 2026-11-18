use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScopePolicy {
    pub visibility: crate::DeclarationVisibility,
    pub parent_lookup: bool,
    pub shadowing: crate::ShadowingPolicy,
}
