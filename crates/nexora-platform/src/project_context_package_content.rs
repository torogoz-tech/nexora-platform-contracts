use crate::{PackageDocumentSnapshot, ProjectContextManifest, WorkTask};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectContextPackageContent {
    pub manifest: Option<ProjectContextManifest>,
    pub documents: Vec<PackageDocumentSnapshot>,
    pub tasks: Vec<WorkTask>,
}
