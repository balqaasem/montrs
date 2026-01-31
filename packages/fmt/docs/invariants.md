# Formatter Package Invariants

## 1. Responsibility
`montrs-fmt` ensures a consistent, comment-preserving code style across the MontRS ecosystem, with special support for `view!` macros.

## 2. Invariants
- **Comment Preservation**: The formatter MUST NOT strip non-doc comments. Preserving developer intent is a priority over "perfect" whitespace.
- **Logical Purity**: The formatter MUST NOT change the logical structure or behavior of the code. It is strictly a whitespace and layout tool.
- **Idempotency**: Formatting a file multiple times with the same configuration must result in zero changes after the first pass.
- **The Cascade of Truth**: Configuration must follow the hierarchical resolution (Workspace -> Project -> File) defined in the "Cascade of Truth".

## 3. Boundary Definitions
- **In-Scope**: Rust code formatting, `view!` macro (RSTML) formatting, comment handling.
- **Out-of-Scope**: Linting, code analysis, auto-fixing of logic errors.

## 4. Agent Guidelines
- When generating code, you can rely on `montrs-fmt` to clean up the layout, but you must provide the comments yourself.
- If a formatting error occurs, check the `FormatError` for specific `AgentError` metadata.
