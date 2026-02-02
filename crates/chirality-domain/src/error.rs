//! Domain error types.

use std::path::PathBuf;
use thiserror::Error;

/// Domain-level errors.
#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Invalid state transition for {entity}: {from} -> {to}")]
    InvalidStateTransition {
        entity: String,
        from: String,
        to: String,
    },

    #[error("Write violation: cannot write to {target_path:?} with scope {scope}: {reason}")]
    WriteViolation {
        target_path: PathBuf,
        scope: String,
        reason: String,
    },

    #[error("Invalid session brief: {reason}")]
    InvalidBrief { reason: String },

    #[error("Human actor required for {operation}")]
    HumanActorRequired { operation: String },

    #[error("Entity not found: {entity_type} with id {id}")]
    NotFound { entity_type: String, id: String },

    #[error("Invalid entity state: {message}")]
    InvalidState { message: String },

    #[error("Precondition failed: {message}")]
    PreconditionFailed { message: String },
}
