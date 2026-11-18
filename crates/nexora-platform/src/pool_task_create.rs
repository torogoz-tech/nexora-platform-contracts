use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolTaskCreate {
    #[serde(default)]
    pub project_id: Option<String>,
    #[serde(flatten)]
    pub definition: crate::TaskCreate,
}
