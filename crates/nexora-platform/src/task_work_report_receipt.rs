use serde::{Deserialize, Serialize};
/// Server-stamped provenance of a report, not a Git verification certificate.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaskWorkReportReceipt {
    pub provenance: crate::TaskWorkProvenance,
    pub credential_id: String,
    pub reported_at: String,
}
