//! # chirality-domain
//!
//! Pure domain core for chirality-runtime. Contains entities, state machines,
//! and domain services with no external dependencies.
//!
//! ## Key Concepts
//!
//! - **Project**: Aggregate root containing decomposition and workspace path
//! - **Package**: Flat scope partition (PKG-###)
//! - **Deliverable**: Primary work unit with 6-state lifecycle (DEL-##.##)
//! - **Document**: The four documents + metadata files
//! - **AgentSession**: Type 1 (PERSONA) or Type 2 (TASK) execution context
//!
//! ## Design Principles
//!
//! - Filesystem IS the state (no hidden database)
//! - Git provides version control and audit trail
//! - Agents have explicit write scopes (WriteGuard)
//! - Human decision rights are sacred

pub mod entities;
pub mod state_machines;
pub mod write_guard;
pub mod brief_parser;
pub mod error;

pub use entities::*;
pub use state_machines::*;
pub use write_guard::*;
pub use error::DomainError;
