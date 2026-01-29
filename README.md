# MontRS: The Deterministic Full-Stack Rust Framework

MontRS is a Rust-native, trait-driven framework for building cross-platform applications. It provides a unified, deterministic environment for web, desktop, and mobile, powered by the performance of Leptos and the safety of Rust's type system.

## Philosophy

MontRS exists because building complex applications requires more than just a UI library. It requires a **predictable architecture**.
- **Determinism**: The same input should always produce the same output, whether in production or testing.
- **Trait-Driven Boundaries**: Features are encapsulated in Modules with explicit interfaces.
- **AI-First**: Built-in metadata and structured snapshots make MontRS applications natively understandable by AI agents.

---

## 🎯 The Golden Path

The "Golden Path" is the recommended workflow for building robust MontRS applications:

1.  **Scaffold**: Start with `montrs new <app-name>` to get a pre-configured workspace.
2.  **Define**: Use `#[derive(Schema)]` to define your data models and validation rules.
3.  **Implement**: Build features as `Module`s. Define `Loader`s for fetching data and `Action`s for mutations.
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

struct HelloModule;

impl Module for HelloModule {
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
- [First 30 Minutes](docs/first-30-minutes.md): **Start here!** Your first onboarding experience.
- [Getting Started](docs/getting-started.md): Your first 10 minutes.
- [The Golden Path](docs/golden-path.md): How to build the right way.
- [Common Pitfalls](docs/pitfalls.md): What to avoid.

### 2. Framework Contributors
*People working **on** MontRS itself.*
- [Architecture Overview](docs/architecture.md): How the engine works.
- [Package Boundaries](docs/packages.md): Responsibility of each crate.
- [Invariants & Philosophy](docs/philosophy.md): The rules we don't break.

### 3. AI Agents
*Machine-readable context for models.*
- [AI Condensed Onboarding](docs/ai-onboarding.md): **Start here!** Rules and invariants for AI agents.
- [AI Usage Guide](packages/llm/README.md): How to use `llm.json` and `tools.json`.
- [Spec Snapshot](docs/spec.md): Understanding the machine-readable project state.
- **Metadata Markers**: Look for `@ai-tool` and `AiError` implementations in the source.

---

## 🛠 Project Structure

| Package | Purpose |
| :--- | :--- |
| [core](packages/core/README.md) | The architectural engine (Modules, Routing, AppSpec). |
| [cli](packages/cli/README.md) | Orchestration, scaffolding, and build tools. |
| [llm](packages/llm/README.md) | AI-first logic, snapshotting, and error tracking. |
| [orm](packages/orm/README.md) | SQL-centric database abstraction. |
| [schema](packages/schema/README.md) | Compile-time validation and data modeling. |
| [test](packages/test/README.md) | Deterministic test runtime and E2E tools. |

---

## License

MontRS is dual-licensed under [Apache-2.0](LICENSE-APACHE) and [MIT](LICENSE-MIT).
