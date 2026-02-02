//! Document entity - content-addressed document within a deliverable.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::{ActorId, ContentHash, DeliverableId, DocumentId};
use crate::entities::DocumentType;

/// Document - a content-addressed document within a deliverable.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: DocumentId,
    pub deliverable_id: DeliverableId,
    pub document_type: DocumentType,
    pub file_path: PathBuf,
    pub content_hash: ContentHash,
    pub state: DocumentState,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: ActorId,
    pub updated_by: ActorId,
}

impl Document {
    pub fn new(
        deliverable_id: DeliverableId,
        document_type: DocumentType,
        file_path: PathBuf,
        content_hash: ContentHash,
        created_by: ActorId,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: DocumentId::new(),
            deliverable_id,
            document_type,
            file_path,
            content_hash,
            state: DocumentState::Draft,
            created_at: now,
            updated_at: now,
            created_by: created_by.clone(),
            updated_by: created_by,
        }
    }

    pub fn update_content(&mut self, new_hash: ContentHash, updated_by: ActorId) {
        self.content_hash = new_hash;
        self.updated_at = Utc::now();
        self.updated_by = updated_by;
    }
}

/// Document lifecycle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DocumentState {
    /// Initial/working state
    Draft,
    /// Human has reviewed
    Reviewed,
    /// Released with deliverable
    Issued,
}

impl DocumentState {
    pub fn can_transition_to(&self, target: &DocumentState) -> bool {
        matches!(
            (self, target),
            (DocumentState::Draft, DocumentState::Reviewed)
                | (DocumentState::Reviewed, DocumentState::Draft) // Can return to draft
                | (DocumentState::Reviewed, DocumentState::Issued)
        )
    }
}
