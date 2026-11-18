use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmailTemplateParameter {
    pub name: String,
    pub description_es: String,
    pub description_en: String,
    pub sample: String,
    pub required: bool,
    pub value_kind: String,
}
