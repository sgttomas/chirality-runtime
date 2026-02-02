//! Strongly-typed identifiers for domain entities.

use serde::{Deserialize, Serialize};
use std::fmt;
use ulid::Ulid;

/// Project identifier (proj:<ULID>)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProjectId(String);

impl ProjectId {
    pub fn new() -> Self {
        Self(format!("proj:{}", Ulid::new()))
    }

    pub fn from_string(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for ProjectId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ProjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Package identifier (PKG-### or pkg:<ULID>)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PackageId(String);

impl PackageId {
    pub fn new() -> Self {
        Self(format!("pkg:{}", Ulid::new()))
    }

    pub fn from_string(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    /// Create from PKG-### format
    pub fn from_legacy(num: u32) -> Self {
        Self(format!("PKG-{:03}", num))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for PackageId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for PackageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Deliverable identifier (DEL-##.## or del:<ULID>)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DeliverableId(String);

impl DeliverableId {
    pub fn new() -> Self {
        Self(format!("del:{}", Ulid::new()))
    }

    pub fn from_string(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    /// Create from DEL-##.## format (package_num.deliverable_num)
    pub fn from_legacy(package_num: u32, deliverable_num: u32) -> Self {
        Self(format!("DEL-{:02}.{:02}", package_num, deliverable_num))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for DeliverableId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for DeliverableId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Document identifier (doc:<ULID>)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DocumentId(String);

impl DocumentId {
    pub fn new() -> Self {
        Self(format!("doc:{}", Ulid::new()))
    }

    pub fn from_string(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for DocumentId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for DocumentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Session identifier (session:<ULID>)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionId(String);

impl SessionId {
    pub fn new() -> Self {
        Self(format!("session:{}", Ulid::new()))
    }

    pub fn from_string(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for SessionId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SessionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Content hash (SHA-256)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ContentHash(String);

impl ContentHash {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        let result = hasher.finalize();
        Self(format!("sha256:{}", hex::encode(result)))
    }

    pub fn from_string(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ContentHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Git commit hash
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CommitHash(String);

impl CommitHash {
    pub fn from_string(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CommitHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Actor identity
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ActorId {
    pub kind: ActorKind,
    pub id: String,
}

impl ActorId {
    pub fn human(id: impl Into<String>) -> Self {
        Self {
            kind: ActorKind::Human,
            id: id.into(),
        }
    }

    pub fn agent(id: impl Into<String>) -> Self {
        Self {
            kind: ActorKind::Agent,
            id: id.into(),
        }
    }

    pub fn system() -> Self {
        Self {
            kind: ActorKind::System,
            id: "system".to_string(),
        }
    }

    pub fn is_human(&self) -> bool {
        matches!(self.kind, ActorKind::Human)
    }
}

impl fmt::Display for ActorId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.kind, self.id)
    }
}

/// Actor kind
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActorKind {
    Human,
    Agent,
    System,
}

impl fmt::Display for ActorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ActorKind::Human => write!(f, "HUMAN"),
            ActorKind::Agent => write!(f, "AGENT"),
            ActorKind::System => write!(f, "SYSTEM"),
        }
    }
}

// Need hex for content hash
mod hex {
    const HEX_CHARS: &[u8; 16] = b"0123456789abcdef";

    pub fn encode(bytes: impl AsRef<[u8]>) -> String {
        let bytes = bytes.as_ref();
        let mut hex = String::with_capacity(bytes.len() * 2);
        for &byte in bytes {
            hex.push(HEX_CHARS[(byte >> 4) as usize] as char);
            hex.push(HEX_CHARS[(byte & 0xf) as usize] as char);
        }
        hex
    }
}
