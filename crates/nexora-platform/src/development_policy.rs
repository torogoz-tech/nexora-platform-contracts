use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DevelopmentPolicy {
    pub repository_id: String,
    pub base_branch: String,
    pub branch_prefix: String,
    pub revision: i64,
}
impl DevelopmentPolicy {
    pub fn valid_ref(value: &str) -> bool {
        !value.is_empty()
            && value.len() <= 180
            && !value.starts_with(['-', '/', '.'])
            && !value.ends_with(['.', '/'])
            && !value.contains("..")
            && !value.contains("//")
            && !value.contains("@{")
            && !value.ends_with(".lock")
            && value.split('/').all(|p| {
                !p.is_empty() && !p.starts_with('.') && !p.ends_with(['.']) && !p.ends_with(".lock")
            })
            && value
                .bytes()
                .all(|c| c.is_ascii_alphanumeric() || b"/._-".contains(&c))
    }
}
