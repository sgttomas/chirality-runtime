//! State machines for domain entities.

use serde::{Deserialize, Serialize};

use crate::entities::AgentClass;
use crate::error::DomainError;

/// Deliverable lifecycle state (from chirality-app).
///
/// ```text
/// OPEN → INITIALIZED → SEMANTIC_READY → IN_PROGRESS → CHECKING → ISSUED
///            │                │               │           │
///            └────────────────┴───────────────┘           │
///                        (StartWork)                      │
///                                         (Reject) ◄──────┘
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DeliverableState {
    /// Folder exists, awaiting initialization
    #[default]
    Open,
    /// 4_DOCUMENTS has run, drafts exist
    Initialized,
    /// CHIRALITY_FRAMEWORK has run (optional)
    SemanticReady,
    /// Active human + agent work
    InProgress,
    /// Under review
    Checking,
    /// Released (terminal)
    Issued,
}

impl DeliverableState {
    /// Check if transition to target state is valid.
    pub fn can_transition_to(&self, target: &DeliverableState) -> bool {
        matches!(
            (self, target),
            (DeliverableState::Open, DeliverableState::Initialized)
                | (DeliverableState::Initialized, DeliverableState::SemanticReady)
                | (DeliverableState::Initialized, DeliverableState::InProgress)
                | (DeliverableState::SemanticReady, DeliverableState::InProgress)
                | (DeliverableState::InProgress, DeliverableState::Checking)
                | (DeliverableState::Checking, DeliverableState::InProgress) // Reject
                | (DeliverableState::Checking, DeliverableState::Issued)
        )
    }

    /// Attempt to transition to a new state.
    pub fn transition_to(self, target: DeliverableState) -> Result<DeliverableState, DomainError> {
        if self.can_transition_to(&target) {
            Ok(target)
        } else {
            Err(DomainError::InvalidStateTransition {
                entity: "Deliverable".to_string(),
                from: format!("{:?}", self),
                to: format!("{:?}", target),
            })
        }
    }

    /// Is this a terminal state?
    pub fn is_terminal(&self) -> bool {
        matches!(self, DeliverableState::Issued)
    }

    /// Can work be done in this state?
    pub fn allows_work(&self) -> bool {
        matches!(
            self,
            DeliverableState::Initialized
                | DeliverableState::SemanticReady
                | DeliverableState::InProgress
        )
    }
}

/// Agent session lifecycle state.
///
/// ```text
/// Type 1 (PERSONA):  CREATED → ACTIVE ←→ PAUSED → COMPLETED/FAILED/CANCELLED
/// Type 2 (TASK):     CREATED → ACTIVE → COMPLETED/FAILED (no pause allowed)
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SessionState {
    #[default]
    Created,
    Active,
    /// Type 1 (PERSONA) only
    Paused,
    Completed,
    Failed,
    Cancelled,
}

impl SessionState {
    /// Check if this session can pause (only PERSONA can pause).
    pub fn can_pause(&self, agent_class: AgentClass) -> bool {
        agent_class == AgentClass::Persona && *self == SessionState::Active
    }

    /// Check if transition to target state is valid.
    pub fn can_transition_to(&self, target: &SessionState, agent_class: AgentClass) -> bool {
        match (self, target, agent_class) {
            // Both types: Created -> Active
            (SessionState::Created, SessionState::Active, _) => true,

            // Both types: Active -> Completed/Failed
            (SessionState::Active, SessionState::Completed, _) => true,
            (SessionState::Active, SessionState::Failed, _) => true,

            // PERSONA only: Active <-> Paused
            (SessionState::Active, SessionState::Paused, AgentClass::Persona) => true,
            (SessionState::Paused, SessionState::Active, AgentClass::Persona) => true,
            (SessionState::Paused, SessionState::Completed, AgentClass::Persona) => true,
            (SessionState::Paused, SessionState::Failed, AgentClass::Persona) => true,

            // Both types: Cancel from non-terminal
            (s, SessionState::Cancelled, _) if !s.is_terminal() => true,

            _ => false,
        }
    }

    /// Attempt to transition to a new state.
    pub fn transition_to(
        self,
        target: SessionState,
        agent_class: AgentClass,
    ) -> Result<SessionState, DomainError> {
        if self.can_transition_to(&target, agent_class) {
            Ok(target)
        } else {
            Err(DomainError::InvalidStateTransition {
                entity: "AgentSession".to_string(),
                from: format!("{:?}", self),
                to: format!("{:?}", target),
            })
        }
    }

    /// Is this a terminal state?
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            SessionState::Completed | SessionState::Failed | SessionState::Cancelled
        )
    }

    /// Is the session currently running?
    pub fn is_active(&self) -> bool {
        matches!(self, SessionState::Active)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deliverable_lifecycle_happy_path() {
        let state = DeliverableState::Open;
        let state = state.transition_to(DeliverableState::Initialized).unwrap();
        let state = state.transition_to(DeliverableState::InProgress).unwrap();
        let state = state.transition_to(DeliverableState::Checking).unwrap();
        let state = state.transition_to(DeliverableState::Issued).unwrap();
        assert!(state.is_terminal());
    }

    #[test]
    fn deliverable_can_skip_semantic() {
        let state = DeliverableState::Initialized;
        assert!(state.can_transition_to(&DeliverableState::InProgress));
    }

    #[test]
    fn deliverable_can_reject_from_checking() {
        let state = DeliverableState::Checking;
        assert!(state.can_transition_to(&DeliverableState::InProgress));
    }

    #[test]
    fn task_session_cannot_pause() {
        let state = SessionState::Active;
        assert!(!state.can_pause(AgentClass::Task));
        assert!(!state.can_transition_to(&SessionState::Paused, AgentClass::Task));
    }

    #[test]
    fn persona_session_can_pause() {
        let state = SessionState::Active;
        assert!(state.can_pause(AgentClass::Persona));
        assert!(state.can_transition_to(&SessionState::Paused, AgentClass::Persona));
    }
}
