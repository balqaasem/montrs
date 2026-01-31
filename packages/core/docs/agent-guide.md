# Agent Guide: montrs-core

This guide is designed to help agents understand and use the core building blocks of the MontRS framework.

## Core Concepts

### 1. The Plate Trait
Every MontRS component is a `Plate`. When building an app, you should define your logic within a struct that implements `Plate`.
- **Metadata**: Always provide a descriptive name and metadata for agent discoverability.
- **Dependencies**: Use `dependencies()` to declare other plates this plate requires. This prevents runtime panics and allows `montrs agent check` to verify your architecture.
- **Initialization**: Use `init` for async setup logic.
- **Routing**: Use `register_routes` to attach loaders and actions.

### 2. Loaders and Actions
- **Loaders**: Read-only operations that fetch data. They should be deterministic and idempotent.
- **Actions**: Write operations that modify state. They should include validation and error handling.
- **Metadata**: Both support `description`, `input_schema`, and `output_schema`. Use these to provide agent-readable specifications.

### 3. Environment and Configuration
- Use `EnvConfig` for type-safe environment variable access.
- `EnvError` provides detailed agent-accessible metadata when variables are missing or invalid.

### 4. Validation
- Use the `Validate` trait for data integrity.
- `ValidationError` is the standard way to report validation failures to the agent for self-correction.

## Agent Usage Patterns

### Generating a New Plate
When asked to create a new feature, implement a `Plate`.
Example:
```rust
pub struct MyPlate;

#[async_trait]
impl Plate<AppConfig> for MyPlate {
    fn name(&self) -> &'static str { "my_plate" }
    fn description(&self) -> &'static str { "Handles user preferences." }
    fn dependencies(&self) -> Vec<&'static str> { vec!["auth_plate"] }
    
    async fn init(&self, ctx: &mut PlateContext<AppConfig>) -> Result<(), Box<dyn StdError + Send + Sync>> {
        // Setup logic
        Ok(())
    }
}
```

### Error Recovery
If you encounter an `AgentError`, check its `error_code` and `suggested_fixes`. The `explanation` field provides a structured reason for the failure.
