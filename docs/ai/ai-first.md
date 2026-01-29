# AI-First Integration Guide

MontRS is the first Rust framework designed from the ground up to be **AI-Native**. This guide explains how AI agents (and the developers who build them) can leverage MontRS's machine-readable infrastructure.

## 🚀 The AI Golden Path

To effectively work with a MontRS project, an AI agent should:

1.  **Consume the Spec**: Read `.llm/llm.json` to understand the project structure, routes, and available tools.
2.  **Use the Tools**: Use `tools.json` to identify CLI commands and framework capabilities that can be called as functions.
3.  **Handle Errors Structurely**: When a task fails, check `.llm/errorfiles/` for the latest `errorfile.json`. It contains the exact error, explanation, and suggested fixes.
4.  **Learn from History**: Look at resolved error files to see diffs of how similar problems were fixed in the past.

## 🛠️ Metadata & Annotations

AI agents should look for the following markers in the codebase:

-   `@ai-tool`: Explicitly marks a function or struct as a tool for the AI.
-   `AiError` implementations: Provides structured data for runtime or compile-time failures.
-   `description()` methods: Found on `Loader`, `Action`, and `Module` traits.

## 📝 Writing AI-First Code

When contributing to MontRS or building an app with it, follow these rules to keep it AI-friendly:

1.  **Always implement `description()`**: Don't leave your loaders and actions as "black boxes."
2.  **Use Structured Errors**: Wrap your errors in types that implement `AiError`.
3.  **Keep READMEs Updated**: The `montrs-llm` package scans package READMEs to curate `tools.json`. Use the standard sections (What it is, What it solves, etc.).

## 🤖 For AI Agents: Self-Correction Loop

If you encounter an error:
1.  Run `montrs spec` to refresh the state.
2.  Read the generated `errorfile.json`.
3.  Apply the `suggested_fix` or use the `explanation` to derive a new one.
4.  Run the command again. If it succeeds, MontRS will record the fix as a diff for your future reference.
