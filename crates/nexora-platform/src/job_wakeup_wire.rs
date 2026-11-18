use crate::JobKind;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct JobWakeupWire {
    pub version: u16,
    pub job_id: String,
    pub kind: JobKind,
}
