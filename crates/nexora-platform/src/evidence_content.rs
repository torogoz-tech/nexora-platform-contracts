use crate::EvidenceRecord;
use serde::{Deserialize, Serialize};
#[derive(Clone, Deserialize, Serialize)]
pub struct EvidenceContent {
    pub record: EvidenceRecord,
    pub data_base64: String,
}
