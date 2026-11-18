use crate::{JobKind, job_wakeup_wire::JobWakeupWire};
use serde::{Deserialize, Deserializer, Serialize, de::Error};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct JobWakeup {
    version: u16,
    job_id: String,
    kind: JobKind,
}

impl JobWakeup {
    pub fn new(job_id: String, kind: JobKind) -> Result<Self, &'static str> {
        if job_id.is_empty()
            || job_id.len() > 128
            || !job_id
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || b"._:-".contains(&byte))
        {
            return Err("job identifier must contain 1..128 safe ASCII characters");
        }

        Ok(Self {
            version: 1,
            job_id,
            kind,
        })
    }

    pub fn version(&self) -> u16 {
        self.version
    }

    pub fn job_id(&self) -> &str {
        &self.job_id
    }

    pub fn kind(&self) -> JobKind {
        self.kind
    }
}

impl<'de> Deserialize<'de> for JobWakeup {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = JobWakeupWire::deserialize(deserializer)?;
        if wire.version != 1 {
            return Err(D::Error::custom("unsupported job wake-up version"));
        }

        Self::new(wire.job_id, wire.kind).map_err(D::Error::custom)
    }
}
