# MontRS: A Deterministic Rust Web Framework

MontRS is a Rust-native, trait-driven web framework for teams that value compile-time correctness, explicit boundaries, and deterministic execution.

## Core Features

- **Fine-grained Reactivity**: Leptos-inspired signals for atomic state management.
- **Remix-style Routing**: File-based routing with explicit loaders and actions.
- **Trait-driven Modularity**: Compose your application using independent modules (pallets).
- **Deterministic Test Runtime**: Boot your entire application spec in-process for fast, reliable tests.
- **SQL-first ORM**: Minimal abstraction over SQL with Drizzle-like ergonomics.

## Quick Start

### Install CLI
```bash
cargo install --path packages/cargo-montrs
```

### Create a project
```bash
cargo montrs new my-app
cd my-app
cargo montrs serve
```

## Documentation

- [Core Architecture](packages/core/README.md)
- [Schema & Validation](packages/schema/README.md)
- [ORM Layer](packages/orm/README.md)
- [Testing Tools](packages/test/README.md)
- [cargo-montrs CLI](packages/cargo-montrs/README.md)
