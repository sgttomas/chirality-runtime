//! Blob store port for large artifact storage.

use async_trait::async_trait;

use chirality_domain::ContentHash;

use crate::error::PortError;

/// Port for content-addressed blob storage.
#[async_trait]
pub trait BlobStorePort: Send + Sync {
    /// Store a blob, returning its content hash.
    async fn store(&self, content: &[u8]) -> Result<ContentHash, PortError>;

    /// Retrieve a blob by content hash.
    async fn retrieve(&self, hash: &ContentHash) -> Result<Vec<u8>, PortError>;

    /// Check if a blob exists.
    async fn exists(&self, hash: &ContentHash) -> Result<bool, PortError>;

    /// Delete a blob.
    async fn delete(&self, hash: &ContentHash) -> Result<(), PortError>;
}
