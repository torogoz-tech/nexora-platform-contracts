use crate::Capability;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemberRole {
    User,
    Supervisor,
    Developer,
    TechnicalLead,
    Pmo,
    Manager,
    SuperAdmin,
}
impl MemberRole {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Supervisor => "supervisor",
            Self::Developer => "developer",
            Self::TechnicalLead => "technical_lead",
            Self::Pmo => "pmo",
            Self::Manager => "manager",
            Self::SuperAdmin => "super_admin",
        }
    }
    pub fn capabilities(self) -> Vec<Capability> {
        if matches!(self, Self::User | Self::Supervisor) {
            return vec![Capability::RequestsRead, Capability::RequestsWrite];
        }
        if self == Self::SuperAdmin {
            return Capability::all();
        }
        let mut capabilities = vec![
            Capability::RequestsRead,
            Capability::RequestsWrite,
            Capability::OrganizationKnowledgeRead,
            Capability::ProjectsRead,
            Capability::RepositoriesRead,
            Capability::KnowledgeRead,
            Capability::WorkflowsRead,
            Capability::TasksRead,
            Capability::TasksWrite,
            Capability::TasksTransition,
            Capability::BacklogsWrite,
            Capability::DocumentsRead,
            Capability::KnowledgeContribute,
            Capability::AnalysisProfilesRead,
            Capability::AnalysisProfilesContribute,
        ];
        if matches!(self, Self::TechnicalLead | Self::Pmo | Self::Manager) {
            capabilities.push(Capability::DocumentsWrite);
            capabilities.push(Capability::KnowledgeWrite);
            capabilities.push(Capability::ApprovalsGrant);
            capabilities.push(Capability::WorkflowAssignmentsManage);
            capabilities.push(Capability::OrganizationKnowledgeWrite);
        }
        if self == Self::TechnicalLead {
            capabilities.push(Capability::MemberProjectsManage);
        }
        for capability in self.configuration_capabilities() {
            if !capabilities.contains(&capability) {
                capabilities.push(capability);
            }
        }
        capabilities
    }
    /// Instance configuration is global; operational task/document grants retain their project scope.
    pub fn configuration_capabilities(self) -> Vec<Capability> {
        if self == Self::TechnicalLead {
            return vec![
                Capability::WorkflowAssignmentsManage,
                Capability::MembersRead,
                Capability::ConnectionsRead,
                Capability::TaskTypesRead,
                Capability::AnalysisProfilesRead,
                Capability::RepositoriesRead,
            ];
        }
        if self == Self::Pmo {
            return vec![Capability::WorkflowAssignmentsManage];
        }
        if !matches!(self, Self::Manager | Self::SuperAdmin) {
            return vec![];
        }
        vec![
            Capability::WorkflowAssignmentsManage,
            Capability::InstallationManage,
            Capability::ActorsManage,
            Capability::KeysManage,
            Capability::ProjectsRead,
            Capability::ProjectsWrite,
            Capability::RepositoriesRead,
            Capability::RepositoriesWrite,
            Capability::RepositoriesSync,
            Capability::ConnectionsManage,
            Capability::WorkflowsRead,
            Capability::WorkflowsWrite,
            Capability::WorkflowsPublish,
        ]
    }
}
