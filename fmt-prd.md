# MontRS-FMT: Product Requirements Document (PRD)

**Version:** 0.1
**Status:** Draft / Architectural Design
**Author:** MontRS Core Team

---

## 1. Executive Summary

`montrs-fmt` is a specialized, high-performance code formatter designed for the MontRS framework. While the Rust ecosystem already possesses powerful tools like `rustfmt` and `prettyplease`, they often fall short when dealing with DSLs embedded in macros—specifically the `view!` macro used extensively in MontRS (via Leptos). 

The primary goal of `montrs-fmt` is to provide a "zero-compromise" formatting experience that respects both standard Rust idioms and the unique structural requirements of MontRS view templates. Unlike its predecessor `leptosfmt`, which relies on a fork of `prettyplease`, `montrs-fmt` will use `prettyplease` as a standard dependency, wrapping it to extend functionality without diverging from the upstream codebase. Crucially, `montrs-fmt` will solve the long-standing issue of non-doc comments being lost within code blocks, ensuring that developer intent is preserved everywhere.

This project is heavily inspired by and built upon the technical foundations established by [leptosfmt](file:///c:/Users/BalqaasemASUS/Documents/GitHub/montrs/.references/leptosfmt) and [prettyplease](file:///c:/Users/BalqaasemASUS/Documents/GitHub/montrs/.references/prettyplease). Therefore, look at them for reference and inspiration.

---

## 2. Vision and Philosophy

### 2.1 Alignment with MontRS Principles
- **Compile-time Correctness**: The formatter must never produce syntactically invalid code. It must be as reliable as the compiler itself.
- **No Hidden Magic**: Formatting rules should be predictable and configurable. The user should understand why their code was transformed.
- **Minimal Abstraction**: Leverage existing, battle-tested libraries (`syn`, `prettyplease`, `rstml`) rather than reinventing the wheel.
- **Deterministic Execution**: Given the same input and configuration, the output must be identical across all platforms (Windows, Linux, WASM).

### 2.2 The "Comments are Content" Doctrine
In many formatters, non-doc comments (`//`) are treated as whitespace and discarded during AST parsing. `montrs-fmt` adopts the philosophy that comments are first-class citizens of the source code. A developer's explanation of a complex logic block is as important as the logic itself. Our architecture will prioritize the preservation and intelligent placement of these comments.

---

## 3. Problem Statement

### 3.1 Limitations of Existing Tools
- **rustfmt**: Bails out on complex macro content, leading to unformatted or poorly formatted `view!` macros. It is also difficult to use as a library due to its dependency on `rustc` internals.
- **prettyplease**: Excellent for generated code but lacks configurability and, by design, discards non-doc comments because it relies on `syn`.
- **leptosfmt**: Successfully formats `view!` macros but relies on a fork of `prettyplease`, making it hard to maintain. It also shares the limitation of losing comments inside Rust code blocks within the macro.

### 3.2 The MontRS Requirement
MontRS developers need a tool that:
1. Formats `view!` macros with HTML-like ergonomics.
2. Formats surrounding Rust code with `rustfmt`-like quality.
3. **Preserves all comments**, including those inside `{}` blocks within a macro.
4. Integrates seamlessly into the `montrs` CLI and VS Code/Editor workflows.

---

## 4. Functional Requirements

### 4.1 Core Formatting
- **Standard Rust**: Format items (structs, enums, functions), statements, and expressions using `prettyplease`.
- **View Macro**: Detect and format `view! { ... }` (and configurable alternatives) using an HTML-aware algorithm.
- **Configurability**: Support `montrs-fmt.toml` for settings like `max_width`, `tab_spaces`, and `closing_tag_style`.

### 4.2 Universal Comment Preservation
- Capture all non-doc comments (`//` and `/* */`) from the source.
- Associate comments with their nearest AST nodes (Preceding, Trailing, or Internal).
- Re-insert comments into the formatted output even when the underlying node was formatted via `prettyplease`.

### 4.3 Integration & Tooling
- **CLI**: A standalone `montrs-fmt` binary and a `montrs fmt` subcommand in the main CLI.
- **Stdin/Stdout**: Support for LSP integration (e.g., `rust-analyzer`'s `overrideCommand`).
- **Check Mode**: A `--check` flag for CI/CD pipelines to verify formatting without modifying files.

---

## 5. Technical Architecture

### 5.1 Dependency Strategy
Instead of forking `prettyplease`, `montrs-fmt` will utilize it as a library. 
- **`syn`**: For parsing Rust code into an AST.
- **`rstml`**: For parsing the contents of the `view!` macro.
- **`prettyplease`**: For formatting standard Rust nodes.
- **`proc-macro2`**: For token stream manipulation and span tracking.

### 5.2 The Formatting Pipeline
1. **Source Ingestion**: Read the raw source string.
2. **Comment Extraction**: Use a custom lexer or `proc-macro2` to find all comments and their byte offsets.
3. **AST Parsing**: Parse the file into a `syn::File`.
4. **Macro Identification**: Traverse the AST to find `view!` macro calls.
5. **Hybrid Formatting**:
    - For non-macro nodes: Wrap the node in a temporary `syn::File`, call `prettyplease::unparse`, and then interleave the preserved comments based on original spans.
    - For `view!` macros: Use a custom `RSTML` printer that is natively comment-aware.
6. **Recomposition**: Combine the formatted chunks into a final string.

### 5.3 Detailed Comment Handling (The "Span-Gap" Algorithm)
To support comments inside code blocks without modifying `prettyplease`, we will implement a "Span-Gap" mapping:
1. Every `syn` node has a `Span`.
2. For any node (e.g., a `Block`), we identify its children.
3. We examine the source text in the "gaps" between the end of child `N` and the start of child `N+1`.
4. If a comment exists in that gap, we store it in a `CommentMap` indexed by the `Span` of the parent and the index of the gap.
5. During formatting, when we reconstruct the parent node, we manually insert the comments between the formatted children.

---

## 6. Configuration Schema (`montrs-fmt.toml`)

The configuration should be compatible with `leptosfmt` where possible but extended for MontRS needs.

```toml
max_width = 100
tab_spaces = 4
indentation_style = "Spaces" # "Tabs" | "Spaces"
newline_style = "Unix" # "Unix" | "Windows"

[view]
closing_tag_style = "SelfClosing" # "Preserve" | "SelfClosing" | "NonSelfClosing"
attr_value_brace_style = "WhenRequired"
macro_names = ["view", "montrs::view"]

[rust]
# Settings passed through to our prettyplease wrapper
preserve_blank_lines = true
```

---

## 7. Implementation Plan

### Phase 1: Core Wrapper
- Setup the project structure: Create the package in `packages/fmt` with the name `montrs-fmt`.
- Implement the basic `prettyplease` wrapper that can format a `syn::File`.
- Build the CLI harness.

### Phase 2: Macro Formatting
- Integrate `rstml`.
- Implement the `view!` macro printer (based on the `leptosfmt` logic but updated for the latest `rstml`).
- Implement HTML-style indentation and attribute wrapping.

### Phase 3: Comment Engine
- Develop the `CommentCollector` to find and store non-doc comments.
- Implement the `CommentInterleaver` that re-inserts comments into `prettyplease` output.
- **Crucial**: Implement the fix for comments inside macro code blocks.

### Phase 4: Integration & Optimization
- Add `montrs.toml` integration.
- Optimize performance for large files.
- Create VS Code extension guidance and `rust-analyzer` configuration docs.

---

## 8. Alignment with MontRS v0.1 PRD

`montrs-fmt` fulfills the requirement for "Tooling & scaffold" mentioned in Section 10 of the MontRS PRD. It ensures that the generated templates and user-authored plates maintain a consistent, readable, and professional standard. By supporting comments everywhere, it upholds the "No hidden magic" and "Explicit boundaries" philosophy, as developers can clearly document their logic boundaries within the reactive UI layer.

---

## 9. Future Considerations

- **Tailwind CSS Support**: Automatically sorting Tailwind classes within `class="..."` attributes, integrated with the `montrs-cli` tailwind config.
- **Doc-test Formatting**: Extending the formatter to handle Rust code blocks inside Markdown doc comments.
- **Plugin System**: Allowing other MontRS plates to register their own macros for specialized formatting (e.g., a `css!` or `sql!` macro).

## 10. Conclusion

`montrs-fmt` is not just a utility; it is a fundamental part of the MontRS developer experience. By solving the technical challenge of comment preservation while maintaining a clean dependency on `prettyplease`, we provide a tool that is both powerful and maintainable. It represents the MontRS commitment to engineering excellence and developer productivity.

---

## 11. References

- **[leptosfmt](file:///c:/Users/BalqaasemASUS/Documents/GitHub/montrs/.references/leptosfmt)**: The original formatter for Leptos `view!` macros. We build upon its logic for `RSTML` parsing and macro detection.
- **[prettyplease](file:///c:/Users/BalqaasemASUS/Documents/GitHub/montrs/.references/prettyplease)**: A minimal `syn` syntax tree pretty-printer. `montrs-fmt` uses this as a standard dependency for high-quality Rust code formatting.
- **[MontRS v0.1 PRD](file:///c:/Users/BalqaasemASUS/Documents/GitHub/montrs/MontRS%20v0.1PRD.md)**: The parent framework specification that `montrs-fmt` supports.