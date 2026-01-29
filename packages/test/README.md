# montrs-test

Deterministic testing utilities for the MontRS framework.

## Key Features

- **Multi-Level Testing**: Unit, Integration, and E2E support.
- **AI-Guided Test Generation**: Metadata for LLMs to generate valid test cases and fixtures.
- **E2E Automation**: Integrated Playwright support via `MontrsDriver`.
- **Deterministic Mocking**: Robust `TestEnv` and `TestRuntime` for predictable results.

## Overview

`montrs-test` provides the infrastructure needed to write robust unit, integration, and end-to-end tests for MontRS applications. It includes tools for mocking, fixture management, and browser automation.

## Key Features

- **Integration Testing**:
  - `Fixture` trait for setup/teardown logic.
  - `run_fixture_test` helper for isolated test execution.
- **Unit Testing**: 
  - `bench` utility for simple performance measurements.
  - Fluent assertions (`expect`).
  - Spies and Mocks (`Spy`, `Mock`).
  - Table-driven tests (`table_test!`).
- **E2E Testing** (via `e2e` feature):
  - `MontrsDriver` wrapper around Playwright (uses `playwright-rs` v0.8.2).
  - `MontrsPlugin` trait for extending driver functionality.
  - Runtime-agnostic design (works with or without MontRS runtime).
- **Environment Mocking**:
  - `TestEnv` for simulating environment variables.
  - `TestRuntime` for in-process application testing.

## Environment Mocking

```rust
use montrs_test::integration::TestEnv;
use montrs_core::EnvConfig;

let env = TestEnv::new();
env.set("DATABASE_URL", "sqlite::memory:");
assert_eq!(env.get_var("DATABASE_URL").unwrap(), "sqlite::memory:");
```

## Integration Testing Usage

```rust
use montrs_test::integration::{Fixture, run_fixture_test};
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

## Unit Testing Usage

### Fluent Assertions

```rust
use montrs_test::unit::expect;

#[test]
fn test_assertions() {
    expect(1 + 1).to_equal(2);
    expect(vec![1, 2]).to_contain(&1);
    expect(true).to_be_true();
}
```

### Benchmarking

```rust
use montrs_test::unit::bench;

#[tokio::test]
async fn test_perf() {
    bench("my_op", 1000, || async {
        // ...
    }).await;
}
```

## E2E Testing Usage

Enable the `e2e` feature in `Cargo.toml`. You may also want to add `playwright` if you need direct access to its types:

```bash
cargo add montrs-test --features e2e
# Optional: Add playwright-rs for direct type usage
cargo add playwright-rs
```

Then use `MontrsDriver` in your tests:

```rust
use montrs_test::e2e::MontrsDriver;

#[tokio::test]
async fn test_home_page() -> anyhow::Result<()> {
    let driver = MontrsDriver::new().await?;
    driver.goto("/").await?;
    let title = driver.page.title().await?;
    assert!(title.contains("MontRS"));
    driver.close().await?;
    Ok(())
}
```

```
