# CLI Package Invariants

## 1. Responsibility
`montrs-cli` is the orchestration hub. It coordinates project lifecycles, builds, and agent-specific metadata generation.

## 2. Invariants
- **Delegated Logic**: Business and framework logic must reside in `montrs-core`. The CLI only handles orchestration and execution.
- **Idempotency**: Scaffolding and generation commands should be idempotent where possible. Running `montrs generate` twice should not result in corrupted state.
- **Agent Synchronization**: Any command that modifies project structure or configuration MUST trigger an update to the `.agent/` directory.
- **Subcommand Isolation**: Commands (e.g., `build`, `serve`, `generate`) should remain modular and not share mutable state.
- **External Tooling**: The CLI must wrap external tools (cargo, wasm-pack) rather than reimplementing their functionality.

## 3. Boundary Definitions
- **In-Scope**: CLI argument parsing, command execution, template management, task running.
- **Out-of-Scope**: Application business logic, core framework trait definitions, direct code compilation.

## 4. Agent Guidelines
- Always verify that a command has a corresponding spec in `command/mod.rs`.
- When adding a new command, ensure it implements proper error reporting via `AgentError`.
- Use the `utils.rs` helpers for common file operations to ensure consistency.
