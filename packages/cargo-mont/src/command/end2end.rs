//! End-to-End test command.
//!
//! This module runs the full end-to-end testing pipeline. It coordinates:
//! 1. Building the application.
//! 2. Starting the backend server.
//! 3. Running the E2E test suite against the running server.
//!
//! It delegates the heavy lifting to `cargo-leptos` but ensures the
//! MontRS configuration is correctly mapped.

use crate::config::MontConfig;

/// Executes the E2E tests.
pub async fn run(headless: bool, keep_alive: bool, browser: Option<String>) -> anyhow::Result<()> {
    // Set environment variables for runtime configuration
    std::env::set_var("MONT_E2E_HEADLESS", headless.to_string());
    if keep_alive {
        std::env::set_var("MONT_E2E_KEEP_ALIVE", "true");
    }
    if let Some(b) = browser {
        std::env::set_var("MONT_E2E_BROWSER", b);
    }

    let config = MontConfig::load()?;
    let leptos_config = config.to_leptos_config(false)?;

    cargo_leptos::command::end2end_all(&leptos_config).await
}
