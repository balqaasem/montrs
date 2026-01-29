# Routing in MontRS: Loaders and Actions

MontRS uses a "Data-First" routing model inspired by Remix. Routes are not just endpoints; they are discrete units of data fetching (`Loader`) and data mutation (`Action`).

## 🛣️ The Router

The `Router` is responsible for matching incoming requests to the appropriate `Plate`. It handles:
- **Static Routes**: `/about`, `/contact`.
- **Dynamic Routes**: `/users/:id`, `/posts/:slug`.
- **Catch-all Routes**: `/*path`.

## 📥 Loaders: Fetching Data

A `Loader` is an async function that runs before a page or component is rendered. It is responsible for providing the necessary state.

```rust
#[async_trait]
impl Loader for UserLoader {
    async fn call(&self, ctx: Context) -> Result<Value> {
        let id = ctx.param("id")?;
        let user = db::get_user(id).await?;
        Ok(json!(user))
    }
}
```

## 📤 Actions: Mutating Data

An `Action` handles non-GET requests (POST, PUT, DELETE). It typically involves validation and persistence.

```rust
#[async_trait]
impl Action for CreateUserAction {
    async fn call(&self, ctx: Context) -> Result<Value> {
        let input: CreateUserInput = ctx.input()?; // Auto-validated via montrs-schema
        let user = db::create_user(input).await?;
        Ok(json!(user))
    }
}
```

## 🔄 The Request Lifecycle

1.  **Match**: The `Router` finds the matching route.
2.  **Authorize**: Middleware checks permissions.
3.  **Validate**: If it's an `Action`, the input is validated against the schema.
4.  **Execute**: The `Loader` or `Action` is called.
5.  **Respond**: The result is serialized and returned.

## 🧩 Composition: Nesting Routes

Routes are typically registered within a `Plate`. You can also nest routers to create complex API structures:

```rust
impl Plate for ApiPlate {
    fn register_routes(&self, router: &mut Router) {
        router.nest("/v1", |v1| {
            v1.add_loader("/status", StatusLoader);
            v1.nest("/users", |users| {
                users.add_loader("/", ListUsersLoader);
                users.add_action("/", CreateUserAction);
                users.add_loader("/:id", GetUserLoader);
            });
        });
    }
}
```

---

## 🔍 Best Practices

1.  **Thin Routes**: Keep your `Loader` and `Action` implementations focused on data transformation and validation. Business logic should live in service functions or plates.
2.  **Explicit Types**: Always specify the input and output types for your routes to enable better agent discovery and type safety.
3.  **Path Parameters**: Use descriptive names for path parameters (e.g., `:user_id` instead of `:id`) to avoid ambiguity in nested routes.

---

## 🤖 Agent-First Routing

By defining routes as structs implementing traits, MontRS allows agents to "see" the interface of every endpoint through the `AppSpec`.

### Common Agent Failure Modes
- **Anti-Pattern**: Putting database queries directly inside the `register_routes` closure.
  - *Fix*: Always implement a `Loader` or `Action` struct.
- **Anti-Pattern**: Hardcoding paths in multiple places.
  - *Fix*: Define route paths as constants if they are used in both the backend and frontend.
- **Anti-Pattern**: Using `Value` for everything.
  - *Fix*: Use concrete structs that implement `Schema` to give the agent (and the compiler) more information.
