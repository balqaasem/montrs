# AI Guide: montrs-core

This guide is designed to help AI models understand and use the core building blocks of the MontRS framework.

## Core Concepts

### 1. The Module Trait
Every MontRS component is a `Module`. When building an app, you should define your logic within a struct that implements `Module`.
- **Metadata**: Always provide a descriptive name and metadata for AI discoverability.
- **Initialization**: Use `init` for async setup logic.
- **Routing**: Use `register_routes` to attach loaders and actions.

### 2. Loaders and Actions
- **Loaders**: Read-only operations that fetch data. They should be deterministic and idempotent.
- **Actions**: Write operations that modify state. They should include validation and error handling.
- **Metadata**: Both support `description`, `input_schema`, and `output_schema`. Use these to provide AI-readable specifications.

### 3. Environment and Configuration
- Use `EnvConfig` for type-safe environment variable access.
- `EnvError` provides detailed AI-accessible metadata when variables are missing or invalid.

### 4. Validation
- Use the `Validate` trait for data integrity.
- `ValidationError` is the standard way to report validation failures to the AI for self-correction.

## AI Usage Patterns

### Generating a New Module
When asked to create a new feature, implement a `Module`.
Example:
```rust
pub struct MyModule;

#[async_trait]
impl Module<AppConfig> for MyModule {
    fn name(&self) -> &'static str { "my_module" }
    fn description(&self) -> &'static str { "Handles user preferences." }
    
    async fn init(&self, ctx: &mut ModuleContext<AppConfig>) -> Result<(), Box<dyn StdError + Send + Sync>> {
        // Setup logic
        Ok(())
    }
}
```

### Error Recovery
If you encounter an `AiError`, check its `error_code` and `suggested_fixes`. The `explanation` field provides a structured reason for the failure.
