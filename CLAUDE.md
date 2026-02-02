# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**chirality-runtime** is a platform for executing multi-agent documentation workflows, built around the principles defined in chirality-app. It provides the runtime infrastructure for agent sessions, deliverable lifecycle management, and filesystem-based state tracking.

**Core Philosophy (from chirality-app):**
- Filesystem IS the state (no hidden database)
- Git provides version control and audit trail
- Agents have explicit write scopes
- Human decision rights are sacred
- Type 0/1/2 agent hierarchy with PERSONA/TASK semantics

**Architecture:** Hexagonal (ports & adapters) with pure domain core, git-based audit trail.

**Stack:** Rust (Axum, git2, tokio) + React/TypeScript frontend. Optional: MinIO for blob storage, Zitadel for OIDC.

## Build & Development Commands

```bash
# Build
cargo build                    # Build all crates
cargo build --release          # Release build

# Test
cargo test                     # Run all tests
cargo test -p chirality-domain # Test specific crate

# Run API server
cargo run -p chirality-api     # Start API server

# Lint
cargo clippy --workspace       # Run clippy
cargo fmt --check              # Check formatting
```

## Codebase Structure

```
crates/
├── chirality-domain/     # Pure domain core - entities, state machines, write guard
├── chirality-ports/      # Port trait definitions (interfaces)
├── chirality-adapters/   # Filesystem, Git, MinIO, Claude API adapters
├── chirality-app/        # Application services (SessionOrchestrator, DeliverableService)
└── chirality-api/        # HTTP API server (Axum)
```

## Key Architectural Patterns

**Hexagonal Architecture:** Domain core has no external dependencies. Adapters implement ports for infrastructure.

**Filesystem as State:** Project truth lives on disk in git repositories. No event sourcing - git commits ARE the audit trail.

**Agent Session Branching:** Each agent session works on an isolated git branch, merged on completion.

**Write Scope Enforcement:** Agents have explicit write zones (WriteGuard validates all writes).

**State Machines:**
- Deliverable: Open → Initialized → SemanticReady → InProgress → Checking → Issued
- AgentSession: Created → Active → Paused (PERSONA only) → Completed/Failed/Cancelled

**Agent Types:**
- Type 0 (Architect): Standards/contracts maintenance
- Type 1 (Manager/PERSONA): Interactive orchestration, human-in-the-loop
- Type 2 (Specialist/TASK): Bounded straight-through execution

## Related Projects

- **chirality-app** (`../chirality-app/`): Documentation-only agent framework; defines agent instructions (AGENT_*.md), templates (INIT-*.md), and workflow standards
- Agent instructions are loaded from the workspace's chirality-app folder at runtime

## Development Guidelines

- Keep domain core pure (no I/O, no external dependencies)
- All infrastructure access goes through ports
- Write scopes must be validated before any filesystem write
- Git commits should include actor attribution
- Test with in-memory port implementations first
