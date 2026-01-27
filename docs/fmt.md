# Formatting in MontRS

MontRS provides a built-in formatter via the `montrs fmt` command. It is designed to be fast, deterministic, and most importantly, respectful of your code's structure and comments.

## The Formatter: `montrs-fmt`

The core of our formatting engine is `montrs-fmt`. It is built on top of `prettyplease`, but addresses several limitations commonly found in Rust formatters.

### Key Innovations

#### 1. Non-Doc Comment Preservation
Standard Rust parsers like `syn` often discard non-doc comments (`//` and `/* ... */`) during the parsing process. This means that a formatter using these parsers will strip out your internal notes and explanations. 

`montrs-fmt` uses a "Span-Gap" algorithm to capture these comments before parsing and re-insert them into the formatted output, ensuring your internal documentation remains intact.

#### 2. RSTML/View Macro Formatting
MontRS uses `view!` macros for UI components. These macros contain HTML-like syntax that standard Rust formatters cannot handle well. `montrs-fmt` includes a specialized printer for RSTML that respects your formatting preferences for tags, attributes, and children.

## Configuration: The Cascade of Truth

We employ a configuration strategy called the **Cascade of Truth**. This allows you to define your settings where they make the most sense for your workflow.

### Resolution Order

1.  **CLI Flags**: Highest priority. (e.g., `montrs fmt --check`)
2.  **`montrs-fmt.toml`**: Project-specific formatter settings. Useful for dedicated formatting tools.
3.  **`montrs.toml` ([fmt] section)**: Unified project settings. **Recommended for most projects.**
4.  **Defaults**: Sensible fallback values.

### Example Configuration

You can add this to your `montrs.toml`:

```toml
[fmt]
max_width = 100
tab_spaces = 4
indentation_style = "Spaces" # or "Tabs"
newline_style = "Unix" # or "Windows"

[fmt.view]
# How to handle empty tags: "SelfClosing" (<div />), "NonSelfClosing" (<div></div>), or "Preserve"
closing_tag_style = "SelfClosing"
# When to use braces for attribute values
attr_value_brace_style = "WhenRequired" # or "Always", "Never"
```

## Why We Chose This Approach

### Wrapping vs. Forking `prettyplease`
We chose to wrap `prettyplease` rather than fork it. This allows us to benefit from upstream improvements and security fixes while layering our custom logic (comment preservation and macro formatting) on top.

### The Span-Gap Algorithm
Preserving comments in a syntax-aware formatter is notoriously difficult. Our Span-Gap approach treats the source code as a series of "tokens" and "gaps". Comments live in the gaps. By tracking the spans of tokens, we can reliably re-insert the gap contents even after the tokens themselves have been rearranged or reformatted.

## CLI Usage

```bash
# Format the entire project
montrs fmt

# Check formatting in CI
montrs fmt --check

# Format a specific directory
montrs fmt --path ./src/components
```
