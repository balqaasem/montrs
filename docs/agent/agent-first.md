# Agent-first integration guide

MontRS is the first Rust framework designed from the ground up to be **Agent-Native**. This guide explains how agents (and the developers who build them) can leverage MontRS's machine-readable infrastructure.

## 🚀 The agent-first golden path

To effectively work with a MontRS project, an agent should:

1.  **Consume the Spec**: Read `.agent/agent.json` to understand the project structure, routes, and available tools.
2.  **Use the Tools**: Use `tools.json` to identify CLI commands and framework capabilities that can be called as functions.
3.  **Handle Errors Structurely**: When a task fails, check `.agent/errorfiles/` for the latest `errorfile.json`. It contains the exact error, explanation, and suggested fixes.
4.  **Learn from History**: Look at resolved error files to see diffs of how similar problems were fixed in the past.

## 🛡️ Architectural Guardrails: Framework Invariants

A key feature of MontRS's agent-native architecture is **Framework Invariants** (`docs/invariants.md`). 

- **Localized Context**: Unlike global documentation, invariants provide high-density rules for a specific framework package or feature.
- **Contract Verification**: Agents use these files as a "contract" to verify that their use of the framework aligns with its intended design.
- **Automated Checking**: The `montrs agent check` command uses these invariants to statically verify that the project's use of the framework remains healthy.

## 🛠️ Metadata & Annotations

Agents should look for the following markers in the codebase:

-   `@agent-tool`: Explicitly marks a function or struct as a tool for the agent.
-   `AgentError` implementations: Provides structured data for runtime or compile-time failures.
-   `description()` methods: Found on `Loader`, `Action`, and `Plate` traits.

## 📝 Writing agent-first code

When contributing to MontRS or building an app with it, follow these rules to keep it agent-friendly:

1.  **Always implement `description()`**: Don't leave your loaders and actions as "black boxes."
2.  **Use Structured Errors**: Wrap your errors in types that implement `AgentError`.
3.  **Keep READMEs Updated**: The `montrs-agent` package scans package READMEs to curate `tools.json`. Use the standard sections (What it is, What it solves, etc.).

## 🤖 For Agents: Self-Correction Loop

If you encounter an error:
1.  Run `montrs spec` to refresh the state.
2.  Read the generated `errorfile.json`.
3.  Apply the `suggested_fix` or use the `explanation` to derive a new one.
4.  Run the command again. If it succeeds, MontRS will record the fix as a diff for your future reference.
