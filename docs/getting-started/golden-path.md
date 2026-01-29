# The Golden Path: Building with MontRS

The "Golden Path" represents the most effective, idiomatic way to build applications with MontRS. Following these patterns ensures your app remains deterministic, testable, and AI-friendly.

## 1. Start with the Schema

Before writing any logic, define the data shape. Use `#[derive(Schema)]` for all inputs and outputs.

```rust
#[derive(Schema, Serialize, Deserialize)]
pub struct CreateTodoInput {
    #[schema(min_len = 1)]
    pub title: String,
}
```

## 2. Define Explicit Loaders and Actions

Avoid mixing data fetching and mutations. Use `Loader` for read operations and `Action` for write operations.

- **Loaders**: Should be side-effect free and return the state needed for a view.
- **Actions**: Should handle validation, persistence, and return the result of the mutation.

## 3. Modularize with Modules

Group related functionality into a `Module`. If the functionality is reusable across projects, package it as a standalone crate.

```rust
impl Module for TodoModule {
    fn register_routes(&self, router: &mut Router) {
        router.add_loader("/todos", ListTodosLoader);
        router.add_action("/todos", CreateTodoAction);
    }
}
```

## 4. Prioritize Determinism in Tests

Use `montrs-test` to write tests that are fast and reliable. Avoid external dependencies in unit tests by using our built-in mocks and fixtures.

## 5. Embrace AI-First Metadata

Always provide a `description()` for your loaders and actions. This small effort pays off significantly when an AI agent needs to understand or modify your code.

## 6. Use the CLI for Orchestration

Rely on `montrs build`, `montrs serve`, and `montrs test` instead of raw `cargo` commands. The CLI handles framework-specific optimizations and context updates.

## 7. Versioned Error Handling

When errors occur, implement `AiError`. This provides the AI (and you) with structured data to resolve issues quickly.
