//! # montrs-test
//!
//! Utilities for deterministic, robust testing of MontRS applications.
//!
//! This crate provides the foundational infrastructure needed to write unit, integration,
//! and end-to-end (E2E) tests. It allows you to:
//!
//! - **Mock Environment Variables**: Use `TestEnv` to simulate different runtime configurations.
//! - **Manage Test Lifecycles**: Use `Fixture` and `run_fixture_test` for setup/teardown logic.
//! - **Run E2E Tests**: Use `MontDriver` (via the `e2e` feature) to control browsers with Playwright.
//! - **Simulate Application Runtime**: Use `TestRuntime` to execute application logic in-process.
//!
//! The E2E capabilities are integrated with `TestRuntime`, allowing you to easily spin up
//! browser tests alongside your integration tests.
//!
//! ## Feature Flags
//!
//! - `e2e`: Enables End-to-End testing capabilities using `playwright-rs`.
//!
//! ## Example
//!
//! ```rust
//! use montrs_test::TestEnv;
//!
//! let env = TestEnv::new();
//! env.set("DATABASE_URL", "sqlite::memory:");
//! assert_eq!(env.get_var("DATABASE_URL").unwrap(), "sqlite::memory:");
//! ```

pub mod unit;

#[cfg(feature = "e2e")]
pub mod e2e;

use montrs_core::env::EnvError;
use montrs_core::{AppConfig, AppSpec, EnvConfig};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// A mock environment configuration provider for testing.
///
/// `TestEnv` allows you to programmaticallly set environment variables that are
/// visible only within the test context, avoiding pollution of the real process environment.
///
/// # Example
///
/// ```rust
/// use montrs_test::TestEnv;
/// use montrs_core::EnvConfig;
///
/// let env = TestEnv::new();
/// env.set("API_KEY", "test-secret");
///
/// assert_eq!(env.get_var("API_KEY").unwrap(), "test-secret");
/// ```
pub struct TestEnv {
    vars: Arc<RwLock<HashMap<String, String>>>,
}

impl Default for TestEnv {
    fn default() -> Self {
        Self::new()
    }
}

impl TestEnv {
    /// Creates a new, empty `TestEnv`.
    pub fn new() -> Self {
        Self {
            vars: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Sets an environment variable for the test session.
    ///
    /// # Arguments
    ///
    /// * `key` - The name of the environment variable.
    /// * `value` - The value to assign.
    pub fn set(&self, key: &str, value: &str) {
        let mut vars = self.vars.write().unwrap();
        vars.insert(key.to_string(), value.to_string());
    }
}

impl EnvConfig for TestEnv {
    /// Retrieves a variable from the test environment.
    ///
    /// Returns `EnvError::MissingKey` if the variable is not set.
    fn get_var(&self, key: &str) -> Result<String, EnvError> {
        let vars = self.vars.read().unwrap();
        vars.get(key)
            .cloned()
            .ok_or_else(|| EnvError::MissingKey(key.to_string()))
    }
}

/// A specialized runtime for executing MontRS components in a test context.
///
/// `TestRuntime` wraps an `AppSpec` and provides facilities to execute closures
/// against that specification. This is useful for integration tests where you need
/// a fully configured application state.
pub struct TestRuntime<C: AppConfig> {
    /// The application specification being tested.
    pub spec: AppSpec<C>,
}

impl<C: AppConfig> TestRuntime<C> {
    /// Creates a new `TestRuntime` with the provided `AppSpec`.
    pub fn new(spec: AppSpec<C>) -> Self {
        Self { spec }
    }

    /// Executes a closure within the test runtime context.
    ///
    /// This method allows you to run code that requires access to the application
    /// configuration and environment.
    ///
    /// # Arguments
    ///
    /// * `f` - A closure that takes a reference to `AppSpec` and returns a result.
    pub async fn execute<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&AppSpec<C>) -> R,
    {
        // In a real implementation, this would set up the reactive context,
        // potentially a tokio task local for the runtime, etc.
        f(&self.spec)
    }
}

#[cfg(feature = "e2e")]
impl<C: AppConfig> TestRuntime<C> {
    /// Creates a new E2E driver instance.
    ///
    /// This provides a convenient way to access the E2E capabilities from within
    /// a test runtime context. It automatically connects to the environment
    /// configured for the test.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let runtime = TestRuntime::new(spec);
    /// let driver = runtime.driver().await?;
    /// driver.goto("/").await?;
    /// ```
    pub async fn driver(&self) -> anyhow::Result<e2e::MontDriver> {
        e2e::MontDriver::new().await
    }
}
