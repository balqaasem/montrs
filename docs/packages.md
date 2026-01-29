# Package Boundaries & Responsibilities

MontRS is organized as a modular workspace. Each package has a specific responsibility and strictly defined boundaries to ensure modularity and ease of maintenance.

---

## 📦 `montrs-core`
- **Responsibility**: Foundational traits (`Module`, `Loader`, `Action`), routing engine, and `AppSpec` definition.
- **Key Components**: `Router`, `Context`, `AppSpec`.
- **Boundary**: It is strictly IO-agnostic. It defines the "Grammar" of how MontRS apps are built.
- **When to modify**: When you need to change how routing works or add new fundamental capabilities to the framework.

## 📦 `montrs-cli`
- **Responsibility**: Scaffolding (`new`), orchestration (`build`, `serve`), and task management.
- **Key Components**: `Config`, `TaskRunner`, `ProjectScaffolder`.
- **Boundary**: It is the "Orchestrator." It depends on `core` and `llm` to understand the project state but does not contain business logic.
- **When to modify**: When adding new CLI commands or improving the developer experience (DX).

## 📦 `montrs-llm`
- **Responsibility**: AI-first logic, snapshot generation (`llm.json`), and versioned error tracking.
- **Key Components**: `LlmManager`, `ErrorRecord`, `ToolScanner`.
- **Boundary**: Acts as a "Sidecar." It scans the codebase (using `core` metadata) to produce machine-optimized context.
- **When to modify**: When improving AI discoverability or changing the `llm.json` schema.

## 📦 `montrs-orm`
- **Responsibility**: Database abstraction, SQL execution, and row mapping.
- **Key Components**: `Database`, `Transaction`, `FromRow`.
- **Boundary**: Handles all persistent data interactions. It provides a unified API that abstracts away the specific database driver (SQLite/Postgres).
- **When to modify**: When adding support for a new database backend or improving the query builder.

## 📦 `montrs-schema`
- **Responsibility**: Declarative validation and metadata generation via proc-macros.
- **Key Components**: `#[derive(Schema)]`, `Validator`.
- **Boundary**: Defines the "Contract" for data structures. It is used by both `core` (for routing) and `orm` (for mapping).
- **When to modify**: When adding new validation rules or expanding metadata capture.

## 📦 `montrs-test`
- **Responsibility**: Deterministic test runtime, fixtures, and E2E drivers.
- **Key Components**: `TestRuntime`, `FixtureManager`.
- **Boundary**: Provides the "Validation Infrastructure." It allows testing of `Loader` and `Action` logic without needing a real network or database.
- **When to modify**: When improving the testability of the framework or adding new mocking capabilities.

---

## How Packages Interact

MontRS follows a **Dependency Inversion** pattern. `montrs-core` defines the traits, and other packages (like `orm` or `schema`) provide implementations or tools that work with those traits.

1.  **CLI** reads **Config** and **Core** to understand the app.
2.  **Core** uses **Schema** to validate data at the boundaries.
3.  **Modules** use **ORM** to persist data.
4.  **LLM** scans everything to produce the **Spec Snapshot**.
