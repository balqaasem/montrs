# Database Backends

MontRS ORM is designed to be backend-agnostic, allowing you to use different databases for development, testing, and production with minimal configuration changes.

## 🚀 Supported Backends

### 1. SQLite (Default for Dev)
- **Crate**: `libsqlite3-sys`
- **Use Case**: Local development, small applications, and edge deployments.
- **Connection String**: `sqlite://data.db` or `sqlite::memory:`

### 2. PostgreSQL
- **Crate**: `tokio-postgres`
- **Use Case**: Production applications requiring high concurrency and advanced features.
- **Connection String**: `postgres://user:pass@localhost/db`

## ⚙️ Configuration

You can specify the database backend in your `montrs.toml`:

```toml
[database]
backend = "sqlite" # or "postgres"
url = "env:DATABASE_URL"
```

## 🔄 Switching Backends

Because MontRS uses a trait-based approach for database interactions, your `Loader` and `Action` code remains the same regardless of the backend:

```rust
// This code works for both SQLite and Postgres
let user = ctx.db()
    .query("SELECT * FROM users WHERE id = $1")
    .bind(1)
    .fetch_one::<User>()
    .await?;
```

## 🧪 Testing with Backends

We recommend using `sqlite::memory:` for unit and integration tests to ensure they are fast and deterministic. For E2E tests, you can use a dedicated test PostgreSQL instance or a file-based SQLite database.
