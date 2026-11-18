#[derive(Clone, Default, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PersonalMcpKeyListQuery {
    pub after: Option<String>,
    pub limit: Option<u32>,
    pub status: Option<String>,
}
