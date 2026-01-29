# AI Guide: montrs-test

This guide helps AI models write and run tests in MontRS.

## Core Concepts

### 1. TestEnv
Mock environment variables to test different configurations.
```rust
let env = TestEnv::new();
env.set("KEY", "VALUE");
```

### 2. Fixture Trait
Define setup and teardown logic for complex integration tests.

### 3. E2E Testing
Uses Playwright for browser automation. 
- **AI Recommendation**: Use `MontrsDriver` to interact with the UI during E2E tests.

### 4. Assertions
Use `expect(value).to_be(expected)` for fluent, AI-readable assertions.

## AI Usage Patterns

### Generating Tests
When asked to add tests for a new feature:
1. Identify if it needs a unit test (logic only) or integration test (requires environment/db).
2. Use `TestEnv` for configuration mocking.
3. Use `run_fixture_test` if resources like databases are needed.

### Debugging Failures
If a test fails with `TEST_EXPECTATION`, compare the actual and expected values provided in the `AiError`.
