# montrs-cli

The orchestration hub for the MontRS framework.

**Target Audiences:** Application Developers, Framework Contributors, AI Agents.

## 1. What this package is
`montrs-cli` is the command-line interface for MontRS. It handles scaffolding, building, serving, and running custom tasks. It can be used as a standalone binary or as a cargo subcommand.

## 2. What problems it solves
- **Scaffolding Friction**: Provides a `new` command with pre-built templates to get started in seconds.
- **Build Complexity**: Orchestrates both frontend (WASM/JS) and backend (Rust) builds into a single command.
- **Task Management**: Replaces complex `Makefile`s or shell scripts with a structured `montrs.toml` task runner.

## 3. What it intentionally does NOT do
- **Project Logic**: It does not contain any of your application's business logic.
- **Direct Compilation**: It calls `cargo`, `wasm-pack`, or other tools rather than compiling code itself.
- **Configuration Parsing**: It delegates framework-level config parsing to `montrs-core`.

## 4. How it fits into the MontRS system
It is the **entry point** for developers. It coordinates the lifecycle of an application from creation to deployment.

## 5. When a user should reach for this package
- When creating a new project (`montrs new`).
- When starting a development server (`montrs serve`).
- When building for production (`montrs build`).
- When generating an AI-ready specification (`montrs spec`).

## 6. Deeper Documentation
- [CLI Command Reference](../../docs/cli.md)
- [Project Configuration](../../docs/getting-started.md#configuration)
- [Task Runner Guide](../../docs/tasks.md)

## 7. Notes for AI Agents
- **Primary Tool**: Use `montrs spec` to get a machine-readable view of the current project state.
- **Automation**: Most CLI commands update the `.llm/` directory automatically.
- **Task Discovery**: Check `montrs.toml` for custom tasks that may be relevant to the current workflow.
- **Determinism**: CLI operations are designed to be idempotent where possible.
