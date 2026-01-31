# Test Package Invariants

## 1. Responsibility
`montrs-test` provides the infrastructure for deterministic unit, integration, and e2e testing.

## 2. Invariants
- **Determinism**: Tests must be reproducible. Any non-deterministic behavior (time, random) must be mockable via `TestRuntime`.
- **Isolation**: Tests should not leak state between runs.
- **Agent-Verifiable**: Testing utilities should provide clear, machine-readable output for agents to verify their own changes.

## 3. Boundary Definitions
- **In-Scope**: Mocking traits, test runners, assertion libraries, e2e orchestration.
- **Out-of-Scope**: Implementation of business logic being tested.
