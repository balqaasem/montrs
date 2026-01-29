# Agent Guide: montrs-fmt

This guide helps agents understand the formatting system of MontRS.

## Core Concepts

### 1. Comment Preservation
Unlike many Rust formatters, `montrs-fmt` is designed to preserve non-doc comments. This is crucial for agents that use comments as metadata or instructions.

### 2. Macro Formatting
Specifically handles `view!` macros. If you are generating UI code, ensure it follows the expected syntax for `view!`.

### 3. The Cascade of Truth
Configuration follows a strict hierarchy: CLI > `montrs-fmt.toml` > `montrs.toml` > Defaults.

## Agent Usage Patterns

### Formatting Generated Code
After generating a large amount of code, you can use `montrs fmt` to ensure it adheres to the project's style. This helps in maintaining consistency and reducing linter errors.

### Troubleshooting Parse Errors
If `montrs fmt` returns a `FMT_PARSE` error, it means the generated code has syntax errors. Use the `explanation` in the `AgentError` to locate the fault.
