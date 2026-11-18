use crate::{PlatformError, WorkflowGraph, WorkflowTransition, WorkflowTransitionStep};
pub(crate) struct WorkflowTransitionRules;
impl WorkflowTransitionRules {
    pub fn validate(graph: &WorkflowGraph, edge: &WorkflowTransition) -> Result<(), PlatformError> {
        let from = graph
            .states
            .iter()
            .find(|state| state.id == edge.from)
            .ok_or(PlatformError::Validation("missing source state"))?;
        let to = graph
            .states
            .iter()
            .find(|state| state.id == edge.to)
            .ok_or(PlatformError::Validation("missing destination state"))?;
        if from.prevent_backtracking
            && (to.progress_order.is_none() || to.progress_order < from.progress_order)
        {
            return Err(PlatformError::Validation(
                "state prevents transitions to an earlier or unranked state",
            ));
        }
        if edge.automatic && (!from.terminal || edge.from != edge.to) {
            return Err(PlatformError::Validation(
                "automatic final transitions must start and remain at the terminal state",
            ));
        }
        if graph
            .transitions
            .iter()
            .filter(|other| other.from == edge.from && other.automatic)
            .count()
            > 1
        {
            return Err(PlatformError::Validation(
                "a terminal state permits only one automatic final pipeline",
            ));
        }
        if edge.steps.len() > 32 {
            return Err(PlatformError::Validation("too many transition steps"));
        }
        if edge.steps.is_empty() {
            return Ok(());
        }
        let mut advanced = false;
        let mut approval = edge.guards.contains(&crate::WorkflowGuard::HumanApproval);
        for step in &edge.steps {
            match step {
                WorkflowTransitionStep::AdvanceState if !advanced => advanced = true,
                WorkflowTransitionStep::PublishRelease if advanced && approval && edge.automatic && edge.steps.last().is_some_and(|last|std::ptr::eq(last,step)) && edge.steps.iter().any(|s|matches!(s,WorkflowTransitionStep::VerifyPullRequestMerged{..}|WorkflowTransitionStep::MergePullRequest{..})) && edge.steps.iter().any(|s|matches!(s,WorkflowTransitionStep::VerifyDeployment{environment,..} if environment=="production")) => {},
                WorkflowTransitionStep::Handoff { workflow_id }
                    if advanced
                        && edge.automatic
                        && !workflow_id.trim().is_empty()
                        && edge
                            .steps
                            .last()
                            .is_some_and(|last| std::ptr::eq(last, step)) => {}
                WorkflowTransitionStep::MergePullRequest { repository_id, target_branch,target_from_parent } if !advanced && approval => {
                    Self::repository(repository_id)?;
                    if if *target_from_parent {!target_branch.is_empty()}else{!crate::DevelopmentPolicy::valid_ref(target_branch)} {
                        return Err(PlatformError::Validation("invalid merge target branch"));
                    }
                }
                WorkflowTransitionStep::CreateBranch {
                    repository_id,
                    base_branch,
                } if !advanced => {
                    Self::repository(repository_id)?;
                    if !crate::DevelopmentPolicy::valid_ref(base_branch) {
                        return Err(PlatformError::Validation("invalid branch base"));
                    }
                }
                WorkflowTransitionStep::GenerateContributorRatings { repository_id } if !advanced => {
                    Self::repository(repository_id)?;
                    if !edge.steps.iter().take_while(|candidate| !std::ptr::eq(*candidate,step)).any(|candidate| matches!(candidate,WorkflowTransitionStep::CreateCodeReview { repository_id: reviewed } if reviewed.is_none() || reviewed == repository_id)) {
                        return Err(PlatformError::Validation("contributor ratings require a preceding matching technical review"));
                    }
                }
                WorkflowTransitionStep::CreateCodeReview { repository_id }
                | WorkflowTransitionStep::VerifyPipeline {
                    repository_id,
                    workflow: _,
                } if !advanced => Self::repository(repository_id)?,
                WorkflowTransitionStep::VerifyPullRequestMerged {
                    repository_id,
                    target_branch,target_from_parent,
                } if !advanced => {
                    Self::repository(repository_id)?;
                    if *target_from_parent && target_branch.is_some() || target_branch
                        .as_deref()
                        .is_some_and(|target| !crate::DevelopmentPolicy::valid_ref(target))
                    {
                        return Err(PlatformError::Validation("invalid merge target branch"));
                    }
                }
                WorkflowTransitionStep::VerifyDeployment {
                    repository_id,
                    environment,
                    target_branch,target_from_parent,
                } if !advanced && !environment.trim().is_empty() => {
                    Self::repository(repository_id)?;
                    if *target_from_parent && target_branch.is_some() || target_branch
                        .as_deref()
                        .is_some_and(|target| !crate::DevelopmentPolicy::valid_ref(target))
                    {
                        return Err(PlatformError::Validation(
                            "invalid deployment target branch",
                        ));
                    }
                }
                WorkflowTransitionStep::Notify { channel,audience }
                    if audience.as_deref().is_none_or(|value|matches!(value,"assignee"|"approvers")) && matches!(channel.as_str(), "email" | "in_app") && (advanced || edge.steps.iter().skip_while(|candidate|!std::ptr::eq(*candidate,step)).any(|candidate|matches!(candidate,WorkflowTransitionStep::RequireApproval))) => {}
                WorkflowTransitionStep::RequireApproval if !advanced && !approval => {
                    approval = true
                }
                WorkflowTransitionStep::VerifyPush { repository_id } if !advanced => {
                    Self::repository(repository_id)?;
                }
                WorkflowTransitionStep::CreatePullRequest {repository_id,source_branch,target_branch,source_from_task,target_from_parent} if !advanced => {
                    Self::repository(repository_id)?;
                    if (*source_from_task && !source_branch.is_empty()) || (!*source_from_task && !crate::DevelopmentPolicy::valid_ref(source_branch))
                        || (*target_from_parent && !target_branch.is_empty()) || (!*target_from_parent && !crate::DevelopmentPolicy::valid_ref(target_branch))
                        || (!*source_from_task && !*target_from_parent && source_branch==target_branch) {
                        return Err(PlatformError::Validation("select a literal branch or its explicit task/parent selector, and distinct valid pull request refs"));
                    }
                }
                _ => {
                    return Err(PlatformError::Validation(
                        "transition steps must be preconditions, one advance_state, then notifications",
                    ));
                }
            }
        }
        if !advanced {
            return Err(PlatformError::Validation(
                "transition steps require advance_state",
            ));
        }
        Ok(())
    }
    fn repository(id: &Option<String>) -> Result<(), PlatformError> {
        if id
            .as_ref()
            .is_some_and(|id| id.is_empty() || id.len() > 128)
        {
            return Err(PlatformError::Validation("invalid step repository"));
        }
        Ok(())
    }
}
