# Agent Package Invariants

## 1. Responsibility
`montrs-agent` is the "describer" and "recorder". It translates the state of a MontRS project into machine-readable formats.

## 2. Invariants
- **Read-Only by Default**: This package should not modify the user's source code directly. It generates metadata, snapshots, and diffs for external application.
- **Inference-Free**: This package must NOT contain logic for calling LLMs or performing AI inference. It only prepares the context for those models.
- **Source of Truth**: The `AgentManager` is the sole authority for generating the `agent.json` specification.
- **Versioned Errors**: Error captures (errorfiles) must be versioned and include the full context required for an agent to propose a fix.

## 3. Boundary Definitions
- **In-Scope**: Project snapshotting, error context capturing, tool specification generation.
- **Out-of-Scope**: Code modification, LLM orchestration, CLI command execution.

## 4. Agent Guidelines
- When extending the snapshot, ensure new fields are optional or have sensible defaults to maintain backward compatibility.
- Ensure that any data captured in `agent.json` is derived directly from the source or CLI metadata.
