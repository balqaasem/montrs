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

## View Macro Formatting

MontRS uses a specialized printer for the `view!` macro to ensure that HTML-like syntax is formatted according to web standards while remaining valid Rust.

### Formatting Rules

1.  **Tag Indentation**: Children of an element are always indented by the configured `tab_spaces`.
2.  **Attribute Alignment**: Attributes are typically placed on the same line as the opening tag unless they exceed `max_width`, in which case they are stacked vertically.
3.  **Self-Closing Tags**: Tags like `<img>`, `<br>`, and `<input>` are automatically self-closed unless configured otherwise.
4.  **Brace Style**: Attribute values like `class={my_class}` are formatted to maintain readability, with a preference for `WhenRequired` to reduce visual noise.

### Example: Before and After

**Original Code:**
```rust
view! { <div class="container" id="main"><h1 >Hello World</h1 ><p>This is a long paragraph that might need some better formatting if it were longer but for now it's just here.</p></div> }
```

**Formatted Output:**
```rust
view! {
    <div class="container" id="main">
        <h1>Hello World</h1>
        <p>
            This is a long paragraph that might need some better formatting if it were longer but
            for now it's just here.
        </p>
    </div>
}
```

---

## Configuration Reference

The `[fmt]` section in your `montrs.toml` (or `montrs-fmt.toml`) supports the following options:

| Option | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `max_width` | `usize` | `100` | Maximum line width before wrapping. |
| `tab_spaces` | `usize` | `4` | Number of spaces per indentation level. |
| `indentation_style` | `String` | `"Spaces"` | `"Spaces"` or `"Tabs"`. |
| `newline_style` | `String` | `"Unix"` | `"Unix"` (\n) or `"Windows"` (\r\n). |
| `use_small_heuristics` | `bool` | `true` | Use shorter formatting for small items (e.g., single-line structs). |

### View-Specific Options (`[fmt.view]`)

| Option | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `closing_tag_style` | `String` | `"SelfClosing"` | `"SelfClosing"` (<div />), `"NonSelfClosing"` (<div></div>), or `"Preserve"`. |
| `attr_value_brace_style` | `String` | `"WhenRequired"` | `"WhenRequired"`, `"Always"`, or `"Never"`. |

---

## Internal Architecture: The Span-Gap Algorithm

For contributors working on `montrs-fmt`, understanding the **Span-Gap Algorithm** is critical.

### The Problem
Most Rust formatters use the `syn` crate to parse code into an Abstract Syntax Tree (AST). However, `syn` is designed for procedural macros and ignores whitespace and non-doc comments. When the AST is printed back to source code, all internal comments are lost.

### Our Solution
1.  **Tokenization**: We use a custom tokenizer that identifies tokens (keywords, identifiers, symbols) and "gaps" (whitespace and comments).
2.  **Span Mapping**: Every token in the AST is mapped to its original source span.
3.  **Gap Capture**: Before formatting, we scan the source file and capture the content of every gap between token spans.
4.  **Re-Injection**: During the printing phase, as we emit tokens from the AST, we check if there were any comments in the "gap" that preceded that token in the original source and re-emit them.

This ensures that `// TODO` or `/* internal note */` comments are never lost, even if the surrounding code is significantly restructured.

## CLI Usage

```bash
# Format the entire project
montrs fmt

# Check formatting in CI
montrs fmt --check

# Format a specific directory
montrs fmt --path ./src/components
```
