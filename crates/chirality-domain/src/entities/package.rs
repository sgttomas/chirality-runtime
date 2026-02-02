//! Package entity - flat scope partition within a project.

use serde::{Deserialize, Serialize};

use super::{PackageId, ProjectId};

/// Package - flat scope partition under a Project.
///
/// From chirality-app: Packages are non-overlapping, non-nested scope partitions.
/// Every scope item from the SOW maps to exactly one Package.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub id: PackageId,
    pub project_id: ProjectId,
    pub label: String,
    pub scope_items: Vec<String>,
    pub folder_name: String,
}

impl Package {
    pub fn new(
        project_id: ProjectId,
        label: impl Into<String>,
    ) -> Self {
        let label = label.into();
        let folder_name = Self::derive_folder_name(&label);
        Self {
            id: PackageId::new(),
            project_id,
            label,
            scope_items: Vec::new(),
            folder_name,
        }
    }

    pub fn with_legacy_id(mut self, num: u32) -> Self {
        self.id = PackageId::from_legacy(num);
        self.folder_name = format!("PKG-{:03}_{}", num, Self::sanitize_label(&self.label));
        self
    }

    pub fn with_scope_items(mut self, items: Vec<String>) -> Self {
        self.scope_items = items;
        self
    }

    fn derive_folder_name(label: &str) -> String {
        format!("{}_{}", PackageId::new(), Self::sanitize_label(label))
    }

    fn sanitize_label(label: &str) -> String {
        label
            .chars()
            .map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' })
            .collect()
    }
}
