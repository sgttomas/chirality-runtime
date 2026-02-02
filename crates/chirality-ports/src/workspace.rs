//! Workspace port for filesystem operations.

use async_trait::async_trait;
use std::path::{Path, PathBuf};

use chirality_domain::{ContentHash, Deliverable};

use crate::error::PortError;

/// Port for filesystem operations within a workspace.
#[async_trait]
pub trait WorkspacePort: Send + Sync {
    /// Read file content.
    async fn read(&self, path: &Path) -> Result<Vec<u8>, PortError>;

    /// Write file content, returning content hash.
    async fn write(&self, path: &Path, content: &[u8]) -> Result<ContentHash, PortError>;

    /// List directory contents.
    async fn list_dir(&self, path: &Path) -> Result<Vec<PathBuf>, PortError>;

    /// Check if path exists.
    async fn exists(&self, path: &Path) -> Result<bool, PortError>;

    /// Compute content hash of a file.
    async fn hash(&self, path: &Path) -> Result<ContentHash, PortError>;

    /// Create directory and parents if needed.
    async fn create_dir_all(&self, path: &Path) -> Result<(), PortError>;

    /// Delete a file.
    async fn delete(&self, path: &Path) -> Result<(), PortError>;

    /// Scaffold deliverable folder structure.
    async fn scaffold_deliverable(&self, deliverable: &Deliverable) -> Result<(), PortError>;
}

/// Filesystem change event for watchers.
#[derive(Debug, Clone)]
pub struct FsChangeEvent {
    pub path: PathBuf,
    pub change_type: FsChangeType,
}

/// Type of filesystem change.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FsChangeType {
    Created,
    Modified,
    Deleted,
}
