# montrs-test

Deterministic testing utilities for the MontRS framework.

## Overview

`montrs-test` provides the infrastructure needed to write robust unit, integration, and end-to-end tests for MontRS applications. It includes tools for mocking, fixture management, and browser automation.

## Key Features

- **Unit Testing**: 
  - `Fixture` trait for setup/teardown logic.
  - `run_fixture_test` helper for isolated test execution.
  - `bench` utility for simple performance measurements.
- **E2E Testing** (via `e2e` feature):
  - `MontDriver` wrapper around Playwright.
  - Automatic server configuration detection.
- **Environment Mocking**:
  - `TestEnv` for simulating environment variables.
  - `TestRuntime` for in-process application testing.

## Unit Testing Usage

```rust
use montrs_test::unit::{Fixture, run_fixture_test};
use async_trait::async_trait;

struct DatabaseFixture;

#[async_trait]
impl Fixture for DatabaseFixture {
    type Context = String; // Example context

    async fn setup(&self) -> anyhow::Result<Self::Context> {
        Ok("connected".to_string())
    }

    async fn teardown(&self, _ctx: &mut Self::Context) -> anyhow::Result<()> {
        // Cleanup logic
        Ok(())
    }
}

#[tokio::test]
async fn test_example() -> anyhow::Result<()> {
    run_fixture_test(DatabaseFixture, |ctx| async move {
        assert_eq!(ctx, "connected");
        Ok(())
    }).await
}
```

## E2E Testing Usage

Enable the `e2e` feature and use `MontDriver`:

```rust
use montrs_test::e2e::MontDriver;

#[tokio::test]
async fn test_homepage() -> anyhow::Result<()> {
    let driver = MontDriver::new().await?;
    driver.goto("/").await?;
    Ok(())
}
```
