# Async Patterns in MontRS ORM

The `montrs-orm` package is built from the ground up to be async-first, integrating seamlessly with the Tokio runtime and MontRS's data-first routing.

## 🧵 Non-Blocking Execution

Every database operation in MontRS is `async`. This ensures that your application remains responsive even under heavy load.

```rust
// Correct usage within a Loader or Action
let posts = ctx.db()
    .query("SELECT * FROM posts")
    .fetch_all::<Post>()
    .await?; // Must be awaited
```

## 🔋 Connection Pooling

MontRS automatically manages a connection pool for you. When you call `ctx.db()`, you are requesting a connection from the pool.

- **Automatic Cleanup**: Connections are returned to the pool once the request lifecycle is complete.
- **Configurable Limits**: You can set the maximum number of connections in `montrs.toml`.

## 🔄 Transactions

For mutations that involve multiple steps, use our async transaction API:

```rust
ctx.db().transaction(|tx| async move {
    tx.execute("INSERT INTO orders ...").await?;
    tx.execute("UPDATE inventory ...").await?;
    Ok(())
}).await?;
```

## 🤖 Agents and Async Code

When an agent generates database code for MontRS, it should always:
1. Use `.await` on all `fetch` and `execute` calls.
2. Handle `Result` types using the `?` operator to leverage the framework's `AgentError` integration.
3. Keep database logic within `Loader` or `Action` implementations to ensure proper context access.
