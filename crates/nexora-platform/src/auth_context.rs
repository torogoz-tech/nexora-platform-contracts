use crate::{ActorKind, AuthMethod, Capability, CapabilityGrant, KeyScope, PlatformError};
/// Trusted context built by credential verification, never deserialized from a client.
#[derive(Debug, Clone)]
pub struct AuthContext {
    actor_id: String,
    kind: ActorKind,
    method: AuthMethod,
    credential_id: String,
    grants: Vec<CapabilityGrant>,
    scope: Option<KeyScope>,
}
impl AuthContext {
    pub fn new(
        actor_id: String,
        kind: ActorKind,
        method: AuthMethod,
        credential_id: String,
        grants: Vec<CapabilityGrant>,
        scope: Option<KeyScope>,
    ) -> Self {
        Self {
            actor_id,
            kind,
            method,
            credential_id,
            grants,
            scope,
        }
    }
    pub fn actor_id(&self) -> &str {
        &self.actor_id
    }
    pub fn kind(&self) -> ActorKind {
        self.kind
    }
    pub fn method(&self) -> AuthMethod {
        self.method
    }
    pub fn credential_id(&self) -> &str {
        &self.credential_id
    }
    pub fn scope(&self) -> Option<&KeyScope> {
        self.scope.as_ref()
    }
    pub fn grants(&self) -> &[CapabilityGrant] {
        &self.grants
    }
    pub fn require(
        &self,
        capability: Capability,
        project: Option<&str>,
    ) -> Result<(), PlatformError> {
        if self.method == AuthMethod::ApiKey && self.scope.is_none() {
            return Err(PlatformError::Forbidden);
        }
        if self
            .scope
            .as_ref()
            .is_some_and(|s| !s.allows(capability, project))
        {
            return Err(PlatformError::Forbidden);
        }
        if self.grants.iter().any(|g| {
            g.capability == capability
                && g.project_id.as_deref().is_none_or(|id| Some(id) == project)
        }) {
            Ok(())
        } else {
            Err(PlatformError::Forbidden)
        }
    }
    pub fn require_human_approval(&self, project: &str) -> Result<(), PlatformError> {
        if self.kind != ActorKind::Human || self.method != AuthMethod::Session {
            return Err(PlatformError::Forbidden);
        }
        self.require(Capability::ApprovalsGrant, Some(project))
    }
}
