# Plates: Composable Applications

In MontRS, a **Plate** is the primary unit of composition. Applications are built by combining multiple plates (often called "reusable plates" when they are shared components). The term "Plate" comes from tectonic plates—the foundational structures that shift and interact to form the Nountain (Mont RS).

## 🧩 What is a Plate?

A `Plate` is a struct that implements the `Plate` trait. Its main job is to register routes (Loaders and Actions) with the `Router`.

```rust
pub struct AuthPlate;

impl Plate for AuthPlate {
    fn register_routes(&self, router: &mut Router) {
        router.add_action("/login", LoginAction);
        router.add_action("/register", RegisterAction);
    }
}
```

## 📦 Reusable Plates

Reusable plates are designed to be shared across projects. Examples include:
- `AuthPlate`: Handles user authentication and sessions.
- `BlogPlate`: Provides a complete blogging engine.
- `AdminPlate`: Generates an administrative dashboard.

## 🛠️ Practical Example: Creating a Reusable Plate

A reusable plate is just a standard plate designed for portability. Here is how you might structure one:

```rust
// packages/plates/blog/src/lib.rs
pub struct BlogPlate {
    pub db_pool: Database,
}

impl Plate for BlogPlate {
    fn name(&self) -> &str { "BlogPlate" }
    
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
2.  **Plate Loading**: Every registered `Plate` is instantiated, often receiving service handles.
3.  **Registration**: `register_routes` is called on each plate to build the global `Router`.
4.  **Validation**: The `AppSpec` is generated and checked for route collisions or missing metadata.
5.  **Runtime**: The server starts, and the `agent.json` spec is updated.

---

## 🤖 Agents and Modularity

Because plates are explicit and trait-based, agents can easily understand and extend the system.

### Common Agent Failure Modes
- **Anti-Pattern**: Forgetting to register a plate in the main `App` entry point.
  - *Fix*: Agents should check `main.rs` to ensure all newly created plates are added to the `App` builder.
- **Anti-Pattern**: Circular dependencies between plates.
  - *Fix*: Plates should communicate via `Loaders` and `Actions` (the API) rather than direct function calls where possible.
- **Anti-Pattern**: Plates that are too large.
  - *Fix*: If a plate has more than 10-15 routes, consider breaking it into smaller, more focused sub-plates.
