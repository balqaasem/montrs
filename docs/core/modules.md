# Modules: Composable Applications

In MontRS, a **Module** is the primary unit of composition. Applications are built by combining multiple modules (often called "reusable modules" when they are shared components).

## 🧩 What is a Module?

A `Module` is a struct that implements the `Module` trait. Its main job is to register routes (Loaders and Actions) with the `Router`.

```rust
pub struct AuthModule;

impl Module for AuthModule {
    fn register_routes(&self, router: &mut Router) {
        router.add_action("/login", LoginAction);
        router.add_action("/register", RegisterAction);
    }
}
```

## 📦 Reusable Modules

Reusable modules are designed to be shared across projects. Examples include:
- `AuthModule`: Handles user authentication and sessions.
- `BlogModule`: Provides a complete blogging engine.
- `AdminModule`: Generates an administrative dashboard.

## 🛠️ Practical Example: Creating a Reusable Module

A reusable module is just a standard module designed for portability. Here is how you might structure one:

```rust
// packages/modules/blog/src/lib.rs
pub struct BlogModule {
    pub db_pool: Database,
}

impl Module for BlogModule {
    fn name(&self) -> &str { "BlogModule" }
    
    fn description(&self) -> Option<String> {
        Some("A complete blogging system with posts and comments.".to_string())
    }

    fn register_routes(&self, router: &mut Router) {
        router.nest("/blog", |blog| {
            blog.add_loader("/posts", ListPostsLoader { db: self.db_pool.clone() });
            blog.add_loader("/posts/:slug", GetPostLoader { db: self.db_pool.clone() });
        });
    }
}
```

---

## 🏗️ Application Bootstrapping

When a MontRS application starts, it follows these steps:

1.  **Initialization**: The `App` instance is created, and core services (DB, Cache) are initialized.
2.  **Module Loading**: Every registered `Module` is instantiated, often receiving service handles.
3.  **Registration**: `register_routes` is called on each module to build the global `Router`.
4.  **Validation**: The `AppSpec` is generated and checked for route collisions or missing metadata.
5.  **Runtime**: The server starts, and the `llm.json` spec is updated.

---

## 🤖 AI and Modularity

Because modules are explicit and trait-based, AI agents can easily understand and extend the system.

### Common AI Failure Modes
- **Anti-Pattern**: Forgetting to register a module in the main `App` entry point.
  - *Fix*: AI agents should check `main.rs` to ensure all newly created modules are added to the `App` builder.
- **Anti-Pattern**: Circular dependencies between modules.
  - *Fix*: Modules should communicate via `Loaders` and `Actions` (the API) rather than direct function calls where possible.
- **Anti-Pattern**: Modules that are too large.
  - *Fix*: If a module has more than 10-15 routes, consider breaking it into smaller, more focused sub-modules.
