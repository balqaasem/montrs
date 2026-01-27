//! # montrs-test
//!
//! Utilities for deterministic, robust testing of MontRS applications.
//!
//! This crate provides the foundational infrastructure needed to write unit, integration,
//! and end-to-end (E2E) tests. It allows you to:
//!
//! - **Mock Environment Variables**: Use `TestEnv` to simulate different runtime configurations.
//! - **Manage Test Lifecycles**: Use `Fixture` and `run_fixture_test` for setup/teardown logic.
//! - **Run E2E Tests**: Use `MontrsDriver` (via the `e2e` feature) to control browsers with Playwright.
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
pub mod integration;

#[cfg(feature = "e2e")]
pub mod e2e;

pub use integration::{Fixture, TestRuntime, TestEnv, run_fixture_test};
pub use unit::{expect, bench, Spy, Mock};
