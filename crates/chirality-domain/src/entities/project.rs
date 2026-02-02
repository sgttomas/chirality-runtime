//! Project entity - aggregate root for a chirality workspace.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::{ActorId, ProjectId};

/// Project - aggregate root containing decomposition and workspace path.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: ProjectId,
    pub name: String,
    pub description: Option<String>,
    pub workspace_path: PathBuf,
    pub decomposition_path: Option<PathBuf>,
    pub created_at: DateTime<Utc>,
    pub created_by: ActorId,
}

impl Project {
    pub fn new(
        name: impl Into<String>,
        workspace_path: PathBuf,
        created_by: ActorId,
    ) -> Self {
        Self {
            id: ProjectId::new(),
            name: name.into(),
            description: None,
            workspace_path,
            decomposition_path: None,
            created_at: Utc::now(),
            created_by,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_decomposition(mut self, path: PathBuf) -> Self {
        self.decomposition_path = Some(path);
        self
    }
}
