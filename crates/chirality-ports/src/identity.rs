//! Identity port for authentication.

use async_trait::async_trait;

use chirality_domain::{ActorId, ActorKind};

use crate::error::PortError;

/// Port for identity and authentication.
#[async_trait]
pub trait IdentityPort: Send + Sync {
    /// Validate a token and return the actor identity.
    async fn validate(&self, token: &str) -> Result<ActorId, PortError>;

    /// Get the actor kind from an actor ID.
    fn actor_kind(&self, actor: &ActorId) -> ActorKind;
}

/// Identity claims from a validated token.
#[derive(Debug, Clone)]
pub struct IdentityClaims {
    pub actor_id: ActorId,
    pub email: Option<String>,
    pub name: Option<String>,
    pub roles: Vec<String>,
}
