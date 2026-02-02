//! # chirality-api
//!
//! HTTP API server for chirality-runtime using Axum.
//!
//! ## Endpoints
//!
//! - `/api/v1/projects` - Project management
//! - `/api/v1/packages` - Package management
//! - `/api/v1/deliverables` - Deliverable lifecycle
//! - `/api/v1/sessions` - Agent session control
//! - `/api/v1/documents` - Document operations

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "chirality_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting chirality-api server...");

    // TODO: Build app state with adapters
    // TODO: Build router with handlers
    // TODO: Start server

    tracing::info!("chirality-api is a work in progress");
    tracing::info!("See the plan at ~/.claude/plans/tidy-crunching-babbage.md for implementation phases");
}
