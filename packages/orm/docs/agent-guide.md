# Agent Guide: montrs-orm

This guide helps agents interact with the database layer of MontRS.

## Core Concepts

### 1. DbBackend Trait
The primary interface for database operations. It supports `execute` for writes and `query` for reads.
- **SQLite**: Use `SqliteBackend` for local, file-based storage.
- **Postgres**: Use `PostgresBackend` for production-grade networked databases.

### 2. FromRow Trait
Automatically maps database rows to Rust structs.
- **Agent Recommendation**: Always derive `FromRow` (or implement it) for your data models.

### 3. ToSql Trait
A bridge for passing parameters safely to different backends.

## Agent Usage Patterns

### Creating a Data Model
```rust
#[derive(FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
}
```

### Executing Queries
When asked to fetch data, use the `query` method and map the result to a struct using `FromRow`.
```rust
let users: Vec<User> = db.query("SELECT * FROM users", &[]).await?;
```

### Error Handling
`DbError` implements `AgentError`. If a query fails, the `error_code` will indicate if it's a `DB_QUERY` syntax error or a `DB_CONNECTION` issue.
