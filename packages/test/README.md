# montrs-test

Deterministic testing utilities for the MontRS ecosystem.

**Target Audiences:** Application Developers, Framework Contributors, Agents.

## 1. What this package is
`montrs-test` provides the infrastructure for writing robust unit, integration, and end-to-end tests. It emphasizes determinism, allowing developers to boot their entire application spec in-process for fast and reliable verification.

## 2. What problems it solves
- **Flaky Tests**: By providing a deterministic `TestRuntime`, it eliminates "it works on my machine" issues caused by timing or environment variance.
- **Complex Setup**: The `Fixture` system automates the setup and teardown of external resources like databases or file systems.
- **E2E Overhead**: Integrated Playwright support via `MontrsDriver` simplifies browser automation for full-stack tests.

## 3. What it intentionally does NOT do
- **Test Execution**: It does not replace `cargo test`; it provides the tools used *within* your tests.
- **Static Analysis**: It does not check code for bugs without running it (use `clippy` for that).
- **Code Coverage**: It does not generate coverage reports (use `cargo-tarpaulin` or similar).

## 4. How it fits into the MontRS system
It is the **validation layer**. It uses the `AppSpec` from `montrs-core` to spin up isolated environments for testing.

## 5. When a user should reach for this package
- When writing unit tests for a `Loader` or `Action`.
- When building an integration test that requires a mock database or environment.
- When creating end-to-end user journey tests using a browser.

## 6. Deeper Documentation
- [Testing Philosophy](../../docs/testing/index.md)
- [Using the TestRuntime](../../docs/testing/index.md#test-runtime)
- [E2E with MontrsDriver](../../docs/testing/index.md#e2e-testing)
- [Table-Driven Testing](../../docs/testing/index.md#table-driven-tests)

## 7. Notes for Agents
- **Deterministic Assertions**: Use `expect(...)` for fluent, human-readable assertions in generated tests.
- **Test Generation**: Refer to the `Plate` and `AppSpec` metadata to understand what inputs and outputs need to be tested.
- **Error Handling**: Look for `TestError` with `AgentError` metadata if a test fixture or driver fails.
- **Isolaton**: Always use `TestEnv` and `run_fixture_test` to ensure that tests do not leak state to the host system.

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
