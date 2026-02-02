//! Port error types.

use std::path::PathBuf;
use thiserror::Error;

/// Errors from port operations.
#[derive(Debug, Error)]
pub enum PortError {
    // Workspace errors
    #[error("File not found: {path:?}")]
    FileNotFound { path: PathBuf },

    #[error("Permission denied: {path:?}")]
    PermissionDenied { path: PathBuf },

    #[error("IO error: {message}")]
    Io { message: String },

    // Git errors
    #[error("Git error: {message}")]
    Git { message: String },

    #[error("Branch not found: {branch}")]
    BranchNotFound { branch: String },

    #[error("Merge conflict in {files:?}")]
    MergeConflict { files: Vec<PathBuf> },

    // Blob store errors
    #[error("Blob not found: {hash}")]
    BlobNotFound { hash: String },

    #[error("Storage error: {message}")]
    Storage { message: String },

    // Agent executor errors
    #[error("Agent execution failed: {message}")]
    AgentExecution { message: String },

    #[error("Agent not found: {name}")]
    AgentNotFound { name: String },

    #[error("Session not found: {id}")]
    SessionNotFound { id: String },

    // Identity errors
    #[error("Invalid token: {reason}")]
    InvalidToken { reason: String },

    #[error("Authentication required")]
    AuthenticationRequired,

    // Generic
    #[error("Internal error: {message}")]
    Internal { message: String },
}

impl From<std::io::Error> for PortError {
    fn from(err: std::io::Error) -> Self {
        PortError::Io {
            message: err.to_string(),
        }
    }
}
