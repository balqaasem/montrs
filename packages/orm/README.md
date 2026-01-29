# montrs-orm

A trait-driven, async-first ORM layer for the MontRS framework.

## Overview

`montrs-orm` provides a unified interface for interacting with different database backends. It abstracts away the differences between database drivers while maintaining performance and type safety.

## Key Features

- **Multi-Backend support**: SQLite and PostgreSQL.
- **Async API**: All database operations are `async` from the ground up.
- **Type-Safe Mapping**: `FromRow` trait for Rust structs.
- **AI-Native Querying**: Metadata hooks for LLMs to understand schema and relationships.

## Usage

```rust
use montrs_orm::{DbBackend, SqliteBackend};

let db = SqliteBackend::new(":memory:")?;
db.execute("CREATE TABLE users (id INTEGER, name TEXT)", &[]).await?;
```
