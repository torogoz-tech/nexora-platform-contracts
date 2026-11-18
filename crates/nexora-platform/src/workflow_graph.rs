use crate::{PlatformError, WorkflowGuard, WorkflowState, WorkflowTransition};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkflowGraph {
    pub states: Vec<WorkflowState>,
    pub transitions: Vec<WorkflowTransition>,
}
impl WorkflowGraph {
    pub fn transition(
        &self,
        state: &str,
        id: &str,
        context: &crate::WorkflowEvaluationContext,
    ) -> Result<&WorkflowTransition, PlatformError> {
        self.validate()?;
        if !self.states.iter().any(|candidate| candidate.id == state) {
            return Err(PlatformError::Conflict(
                "resource state is outside its pinned workflow",
            ));
        }
        let transition = self
            .transitions
            .iter()
            .find(|candidate| candidate.id == id && candidate.from == state)
            .ok_or(PlatformError::Conflict(
                "transition is not allowed in current workflow state",
            ))?;
        if transition.requires_approval() && !context.human_approval_valid {
            return Err(PlatformError::Conflict("human approval is required"));
        }
        crate::workflow_guard_evaluator::WorkflowGuardEvaluator::evaluate(
            &transition.guards,
            context,
        )?;
        Ok(transition)
    }
    pub fn validate_resource_kinds(
        &self,
        kinds: &[crate::WorkflowResourceKind],
    ) -> Result<(), PlatformError> {
        if (self.transitions.iter().any(|edge| {
            edge.guards.contains(&WorkflowGuard::WorkingBranchReported) || !edge.steps.is_empty()
        }) || self
            .states
            .iter()
            .any(|state| state.required_base_branch.is_some()))
            && kinds != [crate::WorkflowResourceKind::Task]
        {
            return Err(PlatformError::Validation(
                "Git state policies and transition actions require a task-only workflow",
            ));
        }
        Ok(())
    }
    pub fn validate(&self) -> Result<(), PlatformError> {
        if !(1..=64).contains(&self.states.len()) || self.transitions.len() > 256 {
            return Err(PlatformError::Validation(
                "workflow requires 1..64 states and at most 256 connections",
            ));
        }
        if self.states.iter().filter(|state| state.initial).count() != 1 {
            return Err(PlatformError::Validation(
                "workflow requires exactly one initial state",
            ));
        }
        let mut states = BTreeSet::new();
        for state in &self.states {
            if state
                .progress_order
                .is_some_and(|order| !(0..=65535).contains(&order))
            {
                return Err(PlatformError::Validation(
                    "progress order must be between 0 and 65535",
                ));
            }
            if state
                .required_base_branch
                .as_ref()
                .is_some_and(|branch| !crate::DevelopmentPolicy::valid_ref(branch))
                || state.prevent_backtracking && state.progress_order.is_none()
            {
                return Err(PlatformError::Validation(
                    "state policy requires a valid base branch and explicit progress order",
                ));
            }
            if !Self::key(&state.id)
                || !Self::name(&state.name)
                || !states.insert(state.id.as_str())
            {
                return Err(PlatformError::Validation(
                    "invalid or duplicate workflow state",
                ));
            }
        }
        let mut transitions = BTreeSet::new();
        for transition in &self.transitions {
            if !Self::key(&transition.id)
                || (!transition.name.is_empty() && !Self::name(&transition.name))
                || !transitions.insert(transition.id.as_str())
                || !states.contains(transition.from.as_str())
                || !states.contains(transition.to.as_str())
                || transition.guards.len() > 32
            {
                return Err(PlatformError::Validation(
                    "invalid or duplicate workflow transition",
                ));
            }
            crate::workflow_transition_rules::WorkflowTransitionRules::validate(self, transition)?;
            let mut guards = BTreeSet::new();
            for guard in &transition.guards {
                if !guards.insert(guard) {
                    return Err(PlatformError::Validation("duplicate workflow guard"));
                }
                if let WorkflowGuard::RequiredField { field } = guard
                    && (!Self::key(field) || field.contains('-'))
                {
                    return Err(PlatformError::Validation(
                        "required field must be a simple field identifier",
                    ));
                }
            }
        }
        let initial = self
            .states
            .iter()
            .find(|state| state.initial)
            .ok_or(PlatformError::Validation("initial state missing"))?;
        let mut reached = BTreeSet::from([initial.id.as_str()]);
        let mut pending = vec![initial.id.as_str()];
        while let Some(current) = pending.pop() {
            for transition in self
                .transitions
                .iter()
                .filter(|transition| transition.from == current)
            {
                if reached.insert(transition.to.as_str()) {
                    pending.push(transition.to.as_str());
                }
            }
        }
        if reached.len() != self.states.len() {
            return Err(PlatformError::Validation(
                "workflow contains unreachable states",
            ));
        }
        if serde_json::to_vec(self)
            .map_err(|_| PlatformError::Validation("invalid workflow encoding"))?
            .len()
            > 262_144
        {
            return Err(PlatformError::Validation(
                "workflow definition exceeds size limit",
            ));
        }
        Ok(())
    }
    fn key(value: &str) -> bool {
        !value.is_empty()
            && value.len() <= 64
            && value
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"_-".contains(&b))
    }
    fn name(value: &str) -> bool {
        !value.trim().is_empty() && value.len() <= 160 && !value.chars().any(char::is_control)
    }
}
