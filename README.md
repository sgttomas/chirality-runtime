# chirality-runtime

A minimal CLI for running chirality-app agent sessions. Pick an agent, and the runtime loads AGENT_*.md instructions as the system prompt, creates a git branch, and starts a Claude API conversation.

## Philosophy

- **Filesystem IS the state** - no hidden database
- **Git provides version control** - commits are the audit trail
- **Agents have explicit write scopes** - enforced by WriteGuard
- **Human decision rights are sacred** - user controls git operations

## Architecture

```
CLI (Terminal)
    │
    ▼
Application (SessionStarter, ConversationLoop)
    │
    ├── Filesystem Adapter (read/write files)
    ├── Git Adapter (create branches)
    └── Claude API Adapter (messages + tools)
```

## Project Structure

```
crates/
├── chirality-domain/     # Pure domain core (entities, state machines)
├── chirality-ports/      # Port trait definitions
├── chirality-adapters/   # Infrastructure implementations
├── chirality-app/        # Application services
└── chirality-api/        # CLI entry point
```

## Development

```bash
# Build
cargo build

# Test
cargo test

# Run
cargo run -p chirality-api

# Lint
cargo clippy --workspace
cargo fmt --check
```

## Status

Work in progress. See the implementation plan for details.

## Related

- **chirality-app** - Documentation-only agent framework defining AGENT_*.md instructions and workflow standards
