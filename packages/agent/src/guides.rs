pub const ARCHITECTURE_GUIDE: &str = r#"
# MontRS Architecture Guide
MontRS is a trait-driven, deterministic web framework built on Leptos 0.8.

## Core Concepts
- **AppSpec**: The single source of truth for the application blueprint.
- **Plate**: A unit of composition (Auth, Blog, etc.) that registers routes and providers.
- **Loaders**: Read-only, idempotent data fetching.
- **Actions**: State-changing mutations.

## How to build a plate
1. Implement the `Plate` trait.
2. Define your `Loader` and `Action` implementations.
3. Register them in `register_routes`.
"#;

pub const DEBUGGING_GUIDE: &str = r#"
# Debugging MontRS
MontRS provides an `errorfile.json` in the `.agent/errorfiles` folder when a build or test fails.
Agents should use these files to understand the context of the error and propose fixes.
"#;
