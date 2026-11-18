use crate::ApprovalRequestSummary;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApprovalRequestPage {
    pub items: Vec<ApprovalRequestSummary>,
    pub next_after: Option<String>,
}
