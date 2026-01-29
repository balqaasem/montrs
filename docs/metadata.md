# Metadata Standards for AI Agents

To ensure that MontRS remains machine-readable, we follow a strict set of metadata standards across the codebase.

## 🏷️ The `@ai-tool` Marker

Any function or struct that is intended to be used as a tool by an AI agent must be marked with the `@ai-tool` comment. This allows `montrs-llm` to automatically curate a `tools.json` file.

```rust
// @ai-tool: Generates a new project from a template.
pub fn scaffold_project(name: &str, template: &str) -> Result<()> { ... }
```

## 📝 Trait Metadata

Core traits (`Module`, `Loader`, `Action`, `Limiter`, `Validate`) include a `description()` method. This should return a human-and-machine-readable summary of the component's purpose.

```rust
impl Loader for MyLoader {
    fn description(&self) -> Option<String> {
        Some("Fetches the latest 10 posts from the database.".to_string())
    }
}
```

## 🧬 Schema Definitions

Use `#[derive(Schema)]` on all input and output structs. This provides the AI with the exact data shape and validation rules required for interaction.

## 🔄 The Curation Process

How does your code end up in the `llm.json` or `tools.json`?

1.  **Scanning**: The `montrs-llm` scanner walks your source code.
2.  **Extraction**: It looks for explicit markers like `@ai-tool` and implementations of traits like `Loader` or `AiError`.
3.  **Heuristics**: If a component lacks explicit metadata, the scanner uses heuristics (like reading the README in the same package) to infer its purpose.
4.  **Serialization**: The extracted data is normalized into a standard JSON format and saved to the `.llm/` directory.

---

## 🔍 Best Practices for Metadata

-   **Be Descriptive**: Instead of `fn get_data()`, use `// @ai-tool: Fetches active user profiles from the primary database.`
-   **Include Constraints**: Mention any rate limits or side effects in the description.
-   **Keep it Up-to-Date**: If you change the behavior of a function, remember to update its `@ai-tool` comment or `description()` method.
