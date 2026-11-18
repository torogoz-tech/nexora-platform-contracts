use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Capability {
    #[serde(rename = "requests:read")]
    RequestsRead,
    #[serde(rename = "requests:write")]
    RequestsWrite,
    #[serde(rename = "workflow-assignments:manage")]
    WorkflowAssignmentsManage,
    #[serde(rename = "members:read")]
    MembersRead,
    #[serde(rename = "member-projects:manage")]
    MemberProjectsManage,

    #[serde(rename = "connections:read")]
    ConnectionsRead,
    #[serde(rename = "task-types:read")]
    TaskTypesRead,
    #[serde(rename = "analysis-profiles:read")]
    AnalysisProfilesRead,

    #[serde(rename = "organization-knowledge:read")]
    OrganizationKnowledgeRead,
    #[serde(rename = "organization-knowledge:write")]
    OrganizationKnowledgeWrite,
    #[serde(rename = "knowledge:contribute")]
    KnowledgeContribute,
    #[serde(rename = "analysis-profiles:contribute")]
    AnalysisProfilesContribute,
    #[serde(rename = "installation:manage")]
    InstallationManage,
    #[serde(rename = "actors:manage")]
    ActorsManage,
    #[serde(rename = "keys:manage")]
    KeysManage,
    #[serde(rename = "projects:read")]
    ProjectsRead,
    #[serde(rename = "projects:write")]
    ProjectsWrite,
    #[serde(rename = "repositories:read")]
    RepositoriesRead,
    #[serde(rename = "repositories:write")]
    RepositoriesWrite,
    #[serde(rename = "repositories:sync")]
    RepositoriesSync,
    #[serde(rename = "connections:manage")]
    ConnectionsManage,
    #[serde(rename = "knowledge:read")]
    KnowledgeRead,
    #[serde(rename = "knowledge:write")]
    KnowledgeWrite,
    #[serde(rename = "workflows:read")]
    WorkflowsRead,
    #[serde(rename = "workflows:write")]
    WorkflowsWrite,
    #[serde(rename = "workflows:publish")]
    WorkflowsPublish,
    #[serde(rename = "tasks:read")]
    TasksRead,
    #[serde(rename = "tasks:write")]
    TasksWrite,
    #[serde(rename = "tasks:transition")]
    TasksTransition,
    #[serde(rename = "backlogs:write")]
    BacklogsWrite,
    #[serde(rename = "documents:read")]
    DocumentsRead,
    #[serde(rename = "documents:write")]
    DocumentsWrite,
    #[serde(rename = "documents:publish")]
    DocumentsPublish,
    #[serde(rename = "approvals:grant")]
    ApprovalsGrant,
}
impl Capability {
    pub fn all() -> Vec<Self> {
        vec![
            Self::RequestsRead,
            Self::RequestsWrite,
            Self::WorkflowAssignmentsManage,
            Self::MembersRead,
            Self::MemberProjectsManage,
            Self::ConnectionsRead,
            Self::TaskTypesRead,
            Self::AnalysisProfilesRead,
            Self::OrganizationKnowledgeRead,
            Self::OrganizationKnowledgeWrite,
            Self::KnowledgeContribute,
            Self::AnalysisProfilesContribute,
            Self::InstallationManage,
            Self::ActorsManage,
            Self::KeysManage,
            Self::ProjectsRead,
            Self::ProjectsWrite,
            Self::RepositoriesRead,
            Self::RepositoriesWrite,
            Self::RepositoriesSync,
            Self::ConnectionsManage,
            Self::KnowledgeRead,
            Self::KnowledgeWrite,
            Self::WorkflowsRead,
            Self::WorkflowsWrite,
            Self::WorkflowsPublish,
            Self::TasksRead,
            Self::TasksWrite,
            Self::TasksTransition,
            Self::BacklogsWrite,
            Self::DocumentsRead,
            Self::DocumentsWrite,
            Self::DocumentsPublish,
            Self::ApprovalsGrant,
        ]
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::RequestsRead => "requests:read",
            Self::RequestsWrite => "requests:write",
            Self::WorkflowAssignmentsManage => "workflow-assignments:manage",
            Self::MembersRead => "members:read",
            Self::MemberProjectsManage => "member-projects:manage",

            Self::ConnectionsRead => "connections:read",
            Self::TaskTypesRead => "task-types:read",
            Self::AnalysisProfilesRead => "analysis-profiles:read",

            Self::OrganizationKnowledgeRead => "organization-knowledge:read",
            Self::OrganizationKnowledgeWrite => "organization-knowledge:write",
            Self::KnowledgeContribute => "knowledge:contribute",
            Self::AnalysisProfilesContribute => "analysis-profiles:contribute",
            Self::InstallationManage => "installation:manage",
            Self::ActorsManage => "actors:manage",
            Self::KeysManage => "keys:manage",
            Self::ProjectsRead => "projects:read",
            Self::ProjectsWrite => "projects:write",
            Self::RepositoriesRead => "repositories:read",
            Self::RepositoriesWrite => "repositories:write",
            Self::RepositoriesSync => "repositories:sync",
            Self::ConnectionsManage => "connections:manage",
            Self::KnowledgeRead => "knowledge:read",
            Self::KnowledgeWrite => "knowledge:write",
            Self::WorkflowsRead => "workflows:read",
            Self::WorkflowsWrite => "workflows:write",
            Self::WorkflowsPublish => "workflows:publish",
            Self::TasksRead => "tasks:read",
            Self::TasksWrite => "tasks:write",
            Self::TasksTransition => "tasks:transition",
            Self::BacklogsWrite => "backlogs:write",
            Self::DocumentsRead => "documents:read",
            Self::DocumentsWrite => "documents:write",
            Self::DocumentsPublish => "documents:publish",
            Self::ApprovalsGrant => "approvals:grant",
        }
    }
}
