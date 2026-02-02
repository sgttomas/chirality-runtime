//! Agent executor port for LLM agent execution.

use async_trait::async_trait;
use std::path::PathBuf;

use chirality_domain::{SessionBrief, SessionOutput, WriteScope};

use crate::error::PortError;

/// Port for executing LLM agents.
#[async_trait]
pub trait AgentExecutorPort: Send + Sync {
    /// Execute a Type 2 (TASK) agent session.
    async fn execute_task(
        &self,
        brief: &SessionBrief,
        context: &ExecutionContext,
    ) -> Result<TaskResult, PortError>;

    /// Start a Type 1 (PERSONA) interactive session.
    async fn start_persona(
        &self,
        agent_name: &str,
        context: &ExecutionContext,
    ) -> Result<PersonaSession, PortError>;

    /// Continue a PERSONA session with human input.
    async fn continue_persona(
        &self,
        session: &PersonaSession,
        input: &str,
    ) -> Result<PersonaResponse, PortError>;
}

/// Context for agent execution.
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    /// Path to the workspace root.
    pub workspace_path: PathBuf,
    /// Agent instructions (content of AGENT_*.md).
    pub agent_instructions: String,
    /// Write scope for the session.
    pub write_scope: WriteScope,
    /// Path to the deliverable (if scoped to one).
    pub deliverable_path: Option<PathBuf>,
    /// Additional context files.
    pub context_files: Vec<PathBuf>,
}

/// Result from a TASK agent execution.
#[derive(Debug, Clone)]
pub struct TaskResult {
    pub success: bool,
    pub outputs: Vec<SessionOutput>,
    pub log: String,
    pub error: Option<String>,
}

/// Handle to an active PERSONA session.
#[derive(Debug, Clone)]
pub struct PersonaSession {
    pub session_id: String,
    pub agent_name: String,
    pub conversation_history: Vec<ConversationTurn>,
}

/// A turn in a PERSONA conversation.
#[derive(Debug, Clone)]
pub struct ConversationTurn {
    pub role: ConversationRole,
    pub content: String,
}

/// Role in a conversation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConversationRole {
    Human,
    Agent,
    System,
}

/// Response from a PERSONA session.
#[derive(Debug, Clone)]
pub struct PersonaResponse {
    pub content: String,
    pub awaiting_input: bool,
    pub outputs: Vec<SessionOutput>,
}
