//! # chirality-ports
//!
//! Port trait definitions for chirality-runtime. Ports define the interfaces
//! that adapters must implement to integrate with external systems.
//!
//! ## Ports
//!
//! - **WorkspacePort**: Filesystem operations (read, write, list, watch)
//! - **GitPort**: Version control operations (commit, branch, merge)
//! - **BlobStorePort**: Large artifact storage (content-addressed)
//! - **AgentExecutorPort**: LLM agent execution
//! - **IdentityPort**: Authentication and authorization

pub mod workspace;
pub mod git;
pub mod blob_store;
pub mod agent_executor;
pub mod identity;
pub mod error;

pub use workspace::*;
pub use git::*;
pub use blob_store::*;
pub use agent_executor::*;
pub use identity::*;
pub use error::*;
