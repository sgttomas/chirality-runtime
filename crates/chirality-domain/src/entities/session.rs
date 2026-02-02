//! AgentSession entity - Type 1 (PERSONA) or Type 2 (TASK) execution context.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::{ActorId, ContentHash, DeliverableId, PackageId, ProjectId, SessionId};
use crate::state_machines::SessionState;
use crate::WriteScope;

/// AgentSession - execution context for an agent.
///
/// From chirality-app:
/// - Type 1 (Manager/PERSONA): Interactive orchestration, can pause
/// - Type 2 (Specialist/TASK): Bounded straight-through execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSession {
    pub id: SessionId,
    pub agent_type: AgentType,
    pub agent_class: AgentClass,
    pub agent_name: String,
    pub scope: SessionScope,
    pub brief: Option<SessionBrief>,
    pub state: SessionState,
    pub write_scope: WriteScope,
    pub outputs: Vec<SessionOutput>,
    pub git_branch: Option<String>,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub started_by: ActorId,
}

impl AgentSession {
    pub fn new_task(
        agent_name: impl Into<String>,
        brief: SessionBrief,
        scope: SessionScope,
        write_scope: WriteScope,
        started_by: ActorId,
    ) -> Self {
        Self {
            id: SessionId::new(),
            agent_type: AgentType::Specialist,
            agent_class: AgentClass::Task,
            agent_name: agent_name.into(),
            scope,
            brief: Some(brief),
            state: SessionState::Created,
            write_scope,
            outputs: Vec::new(),
            git_branch: None,
            started_at: Utc::now(),
            completed_at: None,
            started_by,
        }
    }

    pub fn new_persona(
        agent_name: impl Into<String>,
        agent_type: AgentType,
        scope: SessionScope,
        write_scope: WriteScope,
        started_by: ActorId,
    ) -> Self {
        Self {
            id: SessionId::new(),
            agent_type,
            agent_class: AgentClass::Persona,
            agent_name: agent_name.into(),
            scope,
            brief: None,
            state: SessionState::Created,
            write_scope,
            outputs: Vec::new(),
            git_branch: None,
            started_at: Utc::now(),
            completed_at: None,
            started_by,
        }
    }

    pub fn with_branch(mut self, branch: impl Into<String>) -> Self {
        self.git_branch = Some(branch.into());
        self
    }

    pub fn add_output(&mut self, output: SessionOutput) {
        self.outputs.push(output);
    }

    pub fn complete(&mut self) {
        self.state = SessionState::Completed;
        self.completed_at = Some(Utc::now());
    }

    pub fn fail(&mut self) {
        self.state = SessionState::Failed;
        self.completed_at = Some(Utc::now());
    }
}

/// Agent type following chirality-app's three-layer hierarchy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AgentType {
    /// Type 0: Standards/contracts maintenance (e.g., AGENT_HELPS_HUMANS)
    Architect,
    /// Type 1: Interactive orchestration (e.g., ORCHESTRATOR, WORKING_ITEMS)
    Manager,
    /// Type 2: Bounded task execution (e.g., 4_DOCUMENTS, PREPARATION)
    Specialist,
}

/// Agent class: PERSONA (interactive) or TASK (straight-through).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AgentClass {
    /// Interactive sessions that can pause for human input
    Persona,
    /// Straight-through execution, no mid-run decisions
    Task,
}

/// Session scope: what the session is operating on.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", tag = "type")]
pub enum SessionScope {
    Project { project_id: ProjectId },
    Package { package_id: PackageId },
    Deliverable { deliverable_id: DeliverableId },
}

/// Session brief for Type 2 (TASK) agents.
///
/// From chirality-app: INIT-TASK format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionBrief {
    pub task_definition: String,
    pub scope_description: String,
    pub output_contract: Vec<String>,
    pub constraints: Vec<String>,
    pub success_criteria: Vec<String>,
    pub inputs: serde_json::Value,
}

/// Output from a session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionOutput {
    pub output_type: OutputType,
    pub path: PathBuf,
    pub content_hash: ContentHash,
    pub description: Option<String>,
}

/// Type of output produced by a session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OutputType {
    Document,
    Snapshot,
    Report,
    Metadata,
}
