//! Write scope enforcement for agent sessions.
//!
//! From chirality-app: Agents have explicit write zones. This module
//! validates that all filesystem writes stay within declared scopes.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use crate::entities::DeliverableId;
use crate::error::DomainError;

/// Write scope for an agent session.
///
/// From chirality-app's WRITE_SCOPE header in agent instructions.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", tag = "type")]
pub enum WriteScope {
    /// Read-only (e.g., HELP_HUMAN)
    None,
    /// Only within one deliverable folder
    DeliverableLocal {
        deliverable_id: DeliverableId,
        deliverable_path: PathBuf,
    },
    /// Only to tool roots (e.g., execution/_Aggregation/)
    ToolRootOnly { root_path: PathBuf },
    /// Only project-level metadata (e.g., _COORDINATION.md)
    RepoMetadataOnly { allowed_files: Vec<PathBuf> },
}

/// Result of write validation.
#[derive(Debug, Clone)]
pub enum WriteValidation {
    Allowed,
    Denied(WriteViolation),
}

/// Reason for write denial.
#[derive(Debug, Clone)]
pub struct WriteViolation {
    pub target_path: PathBuf,
    pub scope: String,
    pub reason: String,
}

/// Validates write operations against declared scopes.
pub struct WriteGuard;

impl WriteGuard {
    /// Validate that a write to the target path is allowed.
    pub fn validate_write(scope: &WriteScope, target_path: &Path) -> WriteValidation {
        match scope {
            WriteScope::None => WriteValidation::Denied(WriteViolation {
                target_path: target_path.to_path_buf(),
                scope: "None".to_string(),
                reason: "Agent has no write permission".to_string(),
            }),

            WriteScope::DeliverableLocal {
                deliverable_path, ..
            } => {
                if Self::is_within(target_path, deliverable_path) {
                    WriteValidation::Allowed
                } else {
                    WriteValidation::Denied(WriteViolation {
                        target_path: target_path.to_path_buf(),
                        scope: format!("DeliverableLocal({})", deliverable_path.display()),
                        reason: format!(
                            "Path is outside deliverable folder: {}",
                            deliverable_path.display()
                        ),
                    })
                }
            }

            WriteScope::ToolRootOnly { root_path } => {
                if Self::is_within(target_path, root_path) {
                    WriteValidation::Allowed
                } else {
                    WriteValidation::Denied(WriteViolation {
                        target_path: target_path.to_path_buf(),
                        scope: format!("ToolRootOnly({})", root_path.display()),
                        reason: format!("Path is outside tool root: {}", root_path.display()),
                    })
                }
            }

            WriteScope::RepoMetadataOnly { allowed_files } => {
                if allowed_files.iter().any(|f| f == target_path) {
                    WriteValidation::Allowed
                } else {
                    WriteValidation::Denied(WriteViolation {
                        target_path: target_path.to_path_buf(),
                        scope: "RepoMetadataOnly".to_string(),
                        reason: format!(
                            "Path is not in allowed metadata files: {:?}",
                            allowed_files
                        ),
                    })
                }
            }
        }
    }

    /// Check if child is within parent directory.
    fn is_within(child: &Path, parent: &Path) -> bool {
        // Normalize paths for comparison
        let child_canonical = child.canonicalize().ok();
        let parent_canonical = parent.canonicalize().ok();

        match (child_canonical, parent_canonical) {
            (Some(c), Some(p)) => c.starts_with(&p),
            // If we can't canonicalize, fall back to prefix check
            _ => child.starts_with(parent),
        }
    }

    /// Ensure a write is allowed, returning an error if not.
    pub fn ensure_allowed(scope: &WriteScope, target_path: &Path) -> Result<(), DomainError> {
        match Self::validate_write(scope, target_path) {
            WriteValidation::Allowed => Ok(()),
            WriteValidation::Denied(violation) => Err(DomainError::WriteViolation {
                target_path: violation.target_path,
                scope: violation.scope,
                reason: violation.reason,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn none_scope_denies_all() {
        let scope = WriteScope::None;
        let result = WriteGuard::validate_write(&scope, Path::new("/any/path"));
        assert!(matches!(result, WriteValidation::Denied(_)));
    }

    #[test]
    fn deliverable_local_allows_within() {
        let scope = WriteScope::DeliverableLocal {
            deliverable_id: DeliverableId::from_string("del:test"),
            deliverable_path: PathBuf::from("/project/PKG-01/DEL-01.01"),
        };
        let result = WriteGuard::validate_write(
            &scope,
            Path::new("/project/PKG-01/DEL-01.01/Datasheet.md"),
        );
        assert!(matches!(result, WriteValidation::Allowed));
    }

    #[test]
    fn deliverable_local_denies_outside() {
        let scope = WriteScope::DeliverableLocal {
            deliverable_id: DeliverableId::from_string("del:test"),
            deliverable_path: PathBuf::from("/project/PKG-01/DEL-01.01"),
        };
        let result = WriteGuard::validate_write(
            &scope,
            Path::new("/project/PKG-02/DEL-02.01/Datasheet.md"),
        );
        assert!(matches!(result, WriteValidation::Denied(_)));
    }

    #[test]
    fn tool_root_allows_within() {
        let scope = WriteScope::ToolRootOnly {
            root_path: PathBuf::from("/project/execution/_Aggregation"),
        };
        let result = WriteGuard::validate_write(
            &scope,
            Path::new("/project/execution/_Aggregation/snapshot.json"),
        );
        assert!(matches!(result, WriteValidation::Allowed));
    }
}
