//! Git port for version control operations.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::path::{Path, PathBuf};

use chirality_domain::{ActorId, CommitHash};

use crate::error::PortError;

/// Port for git operations.
#[async_trait]
pub trait GitPort: Send + Sync {
    /// Stage files for commit.
    async fn stage(&self, paths: &[PathBuf]) -> Result<(), PortError>;

    /// Stage all changes.
    async fn stage_all(&self) -> Result<(), PortError>;

    /// Commit staged changes.
    async fn commit(&self, message: &str, author: &ActorId) -> Result<CommitHash, PortError>;

    /// Get current HEAD commit.
    async fn head(&self) -> Result<CommitHash, PortError>;

    /// Get current branch name.
    async fn current_branch(&self) -> Result<String, PortError>;

    /// Create a new branch.
    async fn create_branch(&self, name: &str) -> Result<(), PortError>;

    /// Switch to a branch.
    async fn checkout(&self, branch: &str) -> Result<(), PortError>;

    /// Merge a branch into current.
    async fn merge(&self, branch: &str, message: &str) -> Result<CommitHash, PortError>;

    /// Delete a branch.
    async fn delete_branch(&self, name: &str) -> Result<(), PortError>;

    /// Get commit history for a path.
    async fn log(&self, path: Option<&Path>, limit: usize) -> Result<Vec<CommitInfo>, PortError>;

    /// Create a tag.
    async fn tag(&self, name: &str, message: Option<&str>) -> Result<(), PortError>;
}

/// Information about a git commit.
#[derive(Debug, Clone)]
pub struct CommitInfo {
    pub hash: CommitHash,
    pub message: String,
    pub author_name: String,
    pub author_email: String,
    pub timestamp: DateTime<Utc>,
}
