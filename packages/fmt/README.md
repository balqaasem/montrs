# montrs-fmt

A powerful, non-doc comment preserving formatter for MontRS projects. It wraps `prettyplease` and provides custom formatting for `view!` macros.

## Key Features

- **Non-Doc Comment Preservation**: Unlike standard `prettyplease` or `syn` based formatters, `montrs-fmt` preserves your line and block comments even within code blocks.
- **Macro Formatting**: Intelligent formatting for `view!` macros using a custom RSTML printer.
- **The Cascade of Truth**: Hierarchical configuration system for flexible settings.
- **AI-Native Metadata**: Structured error reporting and formatting hints for LLMs.

## The Cascade of Truth

`montrs-fmt` uses a hierarchical configuration system we call the **Cascade of Truth**. This ensures flexibility while maintaining a single source of truth for your project.

The configuration is resolved in the following order (highest priority first):

1.  **Command Line Arguments**: Flags like `--check` or `--verbose` passed directly to `montrs fmt`.
2.  **`montrs-fmt.toml`**: A standalone configuration file in the project root. This is useful for IDE extensions or CI pipelines that only need formatting settings.
3.  **`montrs.toml` ([fmt] section)**: Formatting settings defined within the main MontRS project configuration file. This is the recommended way to keep your project configuration unified.
4.  **Default Settings**: If no configuration is found, `montrs-fmt` falls back to its sensible defaults.

### Why this approach?

-   **Unified Experience**: Most users prefer having all project settings in one place (`montrs.toml`).
-   **Tooling Compatibility**: Standalone tools and IDE plugins often look for tool-specific config files (`montrs-fmt.toml`). By supporting both, we satisfy both needs.
-   **Explicit Overrides**: Command line arguments allow for quick, one-off overrides (e.g., checking formatting in CI).

## Features

-   **Non-Doc Comment Preservation**: Unlike standard `prettyplease` or `syn` based formatters, `montrs-fmt` preserves your line and block comments even within code blocks.
-   **Macro Formatting**: Intelligent formatting for `view!` macros using a custom RSTML printer.
-   **Configurable**: Support for `max_width`, `tab_spaces`, `indentation_style`, and more.

## Usage

### Via MontRS CLI (Recommended)

```bash
montrs fmt
```

### Options

-   `--check`: Verifies if files are formatted without modifying them.
-   `--path <PATH>`: Specify a file or directory to format (defaults to `.`).
-   `--verbose`: Show detailed output.

## Configuration Example (`montrs.toml`)

```toml
[fmt]
max_width = 120
tab_spaces = 2
indentation_style = "Spaces"

[fmt.view]
closing_tag_style = "SelfClosing"
```
