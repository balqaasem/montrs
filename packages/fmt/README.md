# montrs-fmt

The comment-preserving formatter for MontRS projects.

**Target Audiences:** Application Developers, Framework Contributors, AI Agents.

## 1. What this package is
`montrs-fmt` is a specialized code formatter designed for the MontRS ecosystem. It extends `prettyplease` with support for preserving non-doc comments and intelligently formatting `view!` macros.

## 2. What problems it solves
- **Comment Loss**: Standard `syn`-based formatters often strip comments; `montrs-fmt` preserves them, ensuring that developer intent is maintained.
- **Macro Mess**: Standard `rustfmt` struggles with the complex HTML-in-Rust syntax of `view!`. This package provides a dedicated RSTML printer for clean macro formatting.
- **Config Fatigue**: Implements the "Cascade of Truth," a hierarchical configuration system that unifies project settings.

## 3. What it intentionally does NOT do
- **General Purpose Rust Formatting**: It is optimized for MontRS patterns and `view!` macros, not as a 100% replacement for `rustfmt` in non-MontRS projects.
- **Linting**: It does not perform static analysis or suggest code improvements (use `clippy` for that).
- **Auto-Fixing Logic**: It only changes whitespace and code layout, never the logical structure of the code.

## 4. How it fits into the MontRS system
It is integrated into the CLI as the `montrs fmt` command. It ensures that all projects in the workspace maintain a consistent style.

## 5. When a user should reach for this package
- When their `view!` macros are becoming unreadable.
- When they want to enforce a specific coding style across their team.
- When they need to verify formatting in CI (`montrs fmt --check`).

## 6. Deeper Documentation
- [The Cascade of Truth](../../docs/fmt.md#the-cascade-of-truth) - Understanding how configuration is resolved.
- [View Macro Formatting](../../docs/fmt.md#view-macro-formatting) - Detailed rules and examples for `view!` blocks.
- [Configuration Reference](../../docs/fmt.md#configuration-reference) - Full list of available `[fmt]` settings.
- [Span-Gap Algorithm](../../docs/fmt.md#internal-architecture-the-span-gap-algorithm) - For framework contributors.

## 7. Notes for AI Agents
- **Formatting Constraints**: When generating code for MontRS, follow the rules defined in `montrs.toml` under the `[fmt]` section.
- **Comment Importance**: Always include comments in generated code; `montrs-fmt` will preserve them.
- **Error Handling**: Look for `FormatError` with `AiError` metadata if a formatting operation fails.
- **Idempotency**: Formatting a file twice with the same configuration should result in no changes the second time.
