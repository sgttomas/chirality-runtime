//! Deliverable entity - primary work unit with 6-state lifecycle.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::{DeliverableId, DocumentId, PackageId};
use crate::state_machines::DeliverableState;

/// Deliverable - primary work unit within a Package.
///
/// From chirality-app: Deliverables are the unit of production.
/// Each deliverable has a lifecycle (OPEN → ISSUED) and contains
/// four documents (Datasheet, Specification, Guidance, Procedure).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deliverable {
    pub id: DeliverableId,
    pub package_id: PackageId,
    pub label: String,
    pub deliverable_type: Option<String>,
    pub discipline: Option<String>,
    pub responsible_party: Option<String>,
    pub state: DeliverableState,
    pub folder_path: PathBuf,
    pub documents: Vec<DocumentRef>,
    pub anticipated_artifacts: Vec<String>,
}

impl Deliverable {
    pub fn new(
        package_id: PackageId,
        label: impl Into<String>,
        folder_path: PathBuf,
    ) -> Self {
        Self {
            id: DeliverableId::new(),
            package_id,
            label: label.into(),
            deliverable_type: None,
            discipline: None,
            responsible_party: None,
            state: DeliverableState::Open,
            folder_path,
            documents: Vec::new(),
            anticipated_artifacts: Vec::new(),
        }
    }

    pub fn with_legacy_id(mut self, package_num: u32, deliverable_num: u32) -> Self {
        self.id = DeliverableId::from_legacy(package_num, deliverable_num);
        self
    }

    pub fn with_type(mut self, deliverable_type: impl Into<String>) -> Self {
        self.deliverable_type = Some(deliverable_type.into());
        self
    }

    pub fn with_discipline(mut self, discipline: impl Into<String>) -> Self {
        self.discipline = Some(discipline.into());
        self
    }

    pub fn with_responsible_party(mut self, party: impl Into<String>) -> Self {
        self.responsible_party = Some(party.into());
        self
    }

    pub fn add_document(&mut self, doc_ref: DocumentRef) {
        self.documents.push(doc_ref);
    }
}

/// Reference to a document within a deliverable.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentRef {
    pub id: DocumentId,
    pub document_type: DocumentType,
    pub file_path: PathBuf,
}

/// Types of documents in a deliverable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DocumentType {
    // The four core documents
    Datasheet,
    Specification,
    Guidance,
    Procedure,

    // Metadata files
    Context,      // _CONTEXT.md
    Status,       // _STATUS.md
    Dependencies, // _DEPENDENCIES.md
    References,   // _REFERENCES.md
    Semantic,     // _SEMANTIC.md
}

impl DocumentType {
    /// Get the filename for this document type.
    pub fn filename(&self) -> &'static str {
        match self {
            DocumentType::Datasheet => "Datasheet.md",
            DocumentType::Specification => "Specification.md",
            DocumentType::Guidance => "Guidance.md",
            DocumentType::Procedure => "Procedure.md",
            DocumentType::Context => "_CONTEXT.md",
            DocumentType::Status => "_STATUS.md",
            DocumentType::Dependencies => "_DEPENDENCIES.md",
            DocumentType::References => "_REFERENCES.md",
            DocumentType::Semantic => "_SEMANTIC.md",
        }
    }

    /// Is this a core document (one of the four)?
    pub fn is_core(&self) -> bool {
        matches!(
            self,
            DocumentType::Datasheet
                | DocumentType::Specification
                | DocumentType::Guidance
                | DocumentType::Procedure
        )
    }

    /// Is this a metadata file?
    pub fn is_metadata(&self) -> bool {
        !self.is_core()
    }
}
