# MontRS: The Deterministic Full-Stack Rust Framework

MontRS is a Rust-native, trait-driven framework for building cross-platform applications. It provides a unified, deterministic environment for web, desktop, and mobile, powered by the performance of Leptos and the safety of Rust's type system.

## Philosophy

MontRS exists because building complex applications requires more than just a UI library. It requires a **predictable architecture**.
- **Determinism**: The same input should always produce the same output, whether in production or testing.
- **Trait-Driven Boundaries**: Features are encapsulated in Plates with explicit interfaces.
- **Agent-first**: Built-in metadata and structured snapshots make MontRS applications natively understandable by agents.

---

## 🎯 The Golden Path

The "Golden Path" is the recommended workflow for building robust MontRS applications:

1.  **Scaffold**: Start with `montrs new <app-name>` to get a pre-configured workspace.
2.  **Define**: Use `#[derive(Schema)]` to define your data models and validation rules.
3.  **Implement**: Build features as `Plate`s. Define `Loader`s for fetching data and `Action`s for mutations.
4.  **Verify**: Use the `TestRuntime` for in-process, deterministic testing of your entire application spec.
5.  **Ship**: Deploy to your target (Web, Server, or Desktop) using `montrs build`.

---

## 🧠 How to Think in MontRS

- **Everything is a Trait**: If you want to change behavior (ORM, Auth, Rendering), you implement a trait.
- **Loaders are for Reading, Actions are for Writing**: This clear separation simplifies state management and debugging.
- **The AppSpec is Truth**: Your entire application is defined by a serializable `AppSpec`, making it portable and inspectable.
- **No Magic**: We prefer explicit registration over reflection or global state.

---

## 🚀 Minimal Example

```rust
use montrs::prelude::*;

#[derive(Schema, Serialize, Deserialize)]
struct Greeting {
    #[schema(min_len = 3)]
    name: String,
}

struct HelloPlate;

impl Plate for HelloPlate {
    fn register_routes(&self, router: &mut Router) {
        router.add_loader("/hello", HelloLoader);
    }
}

#[async_trait]
impl Loader for HelloLoader {
    async fn call(&self, _ctx: Context) -> Result<Value> {
        Ok(json!({ "message": "Hello from MontRS!" }))
    }
}
```

---

## 👥 Documentation for Every Audience

### 1. Application Developers
*People building apps **with** MontRS.*
- [First 30 Minutes](docs/getting-started/first-30-minutes.md): **Start here!** Your first onboarding experience.
- [Introduction](docs/getting-started/introduction.md): Your first 10 minutes.
- [The Golden Path](docs/getting-started/golden-path.md): How to build the right way.
- [Common Mistakes](docs/guides/common-mistakes.md): Avoid frequent pitfalls and architectural anti-patterns.
  
  ### 2. Framework Contributors
*People working **on** MontRS itself.*
- [Architecture Overview](docs/architecture/overview.md): How the engine works.
- [Package Boundaries](docs/architecture/packages.md): Responsibility of each crate.
- [Invariants & Philosophy](docs/architecture/philosophy.md): The rules we don't break.

### 3. Agents
*Machine-readable context for models.*
- [Agent-first design](docs/agent/agent-first.md): Principles of machine-readability.
- [Agent Usage Guide](packages/agent/README.md): How to use `agent.json` and `tools.json`.
- [Spec Snapshot](docs/agent/spec.md): Understanding the machine-readable project state.
- **Metadata Markers**: Look for `@agent-tool` and `AgentError` implementations in the source.

---

## 🛠 Project Structure

| Package | Purpose |
| :--- | :--- |
| [core](packages/core/README.md) | The architectural engine (Plates, Routing, AppSpec). |
| [cli](packages/cli/README.md) | Orchestration, scaffolding, and build tools. |
| [agent](packages/agent/README.md) | Agent-first logic, snapshotting, and error tracking. |
| [orm](packages/orm/README.md) | SQL-centric database abstraction. |
| [schema](packages/schema/README.md) | Compile-time validation and data modeling. |
| [test](packages/test/README.md) | Deterministic test runtime and E2E tools. |

---

## License

MontRS is dual-licensed under [Apache-2.0](LICENSE-APACHE) and [MIT](LICENSE-MIT).
