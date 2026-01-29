pub const ARCHITECTURE_GUIDE: &str = r#"
# MontRS Architecture Guide
MontRS is a trait-driven, deterministic web framework built on Leptos 0.8.

## Core Concepts
- **AppSpec**: The single source of truth for the application blueprint.
- **Module**: A unit of composition (Auth, Blog, etc.) that registers routes and providers.
- **Loaders**: Read-only, idempotent data fetching.
- **Actions**: State-changing mutations.

## How to build a module
1. Implement the `Module` trait.
2. Define your `Loader` and `Action` implementations.
3. Register them in `register_routes`.
"#;

pub const DEBUGGING_GUIDE: &str = r#"
# Debugging MontRS
MontRS provides an `errorfile.txt` in the `.llm` folder when a build or test fails.
AI models should use this file to understand the context of the error and propose fixes.
"#;
