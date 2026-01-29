# First 30 Minutes with MontRS

Welcome! This guide is designed to take you from "What is this?" to "I've built something" in exactly half an hour. We'll focus on the core concepts and the "Golden Path" of development.

---

## 1. What Is MontRS? (2–3 minutes)

MontRS is a Rust-native, trait-driven web framework built for teams that value **compile-time correctness**, **explicit boundaries**, and **deterministic execution**. It exists to solve the problem of "architectural drift"—where large apps become hard to reason about over time.

**MontRS is great for:**
- Complex web applications with deep data requirements.
- Systems that need to be "AI-First" (easily understood by LLMs).
- Projects where testing and reliability are non-negotiable.

**MontRS intentionally avoids:**
- Implicit "magic" (like global side effects or hidden discovery).
- Overly complex macros for business logic.
- Tight coupling between the UI and the data layer.

**Targets:** You can currently target **Web (WASM)** and **Native Desktop/Server** environments.

---

## 2. The MontRS Mental Model (5 minutes)

To build effectively with MontRS, you only need to understand four core concepts:

1.  **Signals**: Fine-grained reactivity. Instead of re-rendering everything, MontRS updates only the specific parts of the UI that change when a "Signal" value is updated.
2.  **Modules**: The unit of organization. Your app is a collection of independent modules (Pallets) that register their own routes and logic.
3.  **Routes (Loaders & Actions)**: The boundaries of your app.
    -   **Loaders** are for reading data (GET).
    -   **Actions** are for writing/mutating data (POST/PUT/DELETE).
4.  **AppSpec**: A machine-readable "blueprint" of your entire app. It's how MontRS (and AI agents) knows exactly what your app can do without running it.

---

## 3. Create Your First App (5 minutes)

First, install the CLI if you haven't already:
```bash
cargo install --path packages/cli
```

Now, scaffold a new project:
```bash
montrs new my-app
cd my-app
```

### The Generated Structure
-   `src/main.rs`: The entry point where your app and modules are initialized.
-   `src/modules/`: This is where your business logic lives.
-   `montrs.toml`: Your project configuration.
-   `.llm/`: (Auto-generated) Contains the `llm.json` specification for AI context.

Typically, you'll spend 90% of your time in `src/modules/`.

---

## 4. Build Something Small but Real (10 minutes)

Let's build a simple **Counter with Persistence**. We want a button that increments a number and saves it to a (simulated) database.

### Step 1: Define the Schema
In MontRS, we always start with the data shape.
```rust
#[derive(Schema, Serialize, Deserialize)]
pub struct CounterState {
    pub count: i32,
}
```

### Step 2: Create a Loader (Read)
A Loader fetches the current state.
```rust
#[async_trait]
impl Loader for GetCounterLoader {
    async fn call(&self, ctx: Context) -> Result<Value> {
        // In a real app, you'd fetch from ctx.db()
        Ok(json!({ "count": 0 })) 
    }
}
```

### Step 3: Create an Action (Write)
An Action handles the increment.
```rust
#[async_trait]
impl Action for IncrementAction {
    async fn call(&self, ctx: Context) -> Result<Value> {
        let current: CounterState = ctx.input()?;
        Ok(json!({ "count": current.count + 1 }))
    }
}
```

### Step 4: Use Signals in the View
In your component, you'd use a signal to track the local state:
```rust
let (count, set_count) = create_signal(0);

view! {
    <div>
        <p>"Count is: " {count}</p>
        <button on:click=move |_| {
            // Call the IncrementAction here
            set_count.update(|n| *n += 1);
        }>"Increment"</button>
    </div>
}
```

**Why this way?** By separating the *Read* (Loader), the *Write* (Action), and the *UI* (Signal), your code remains easy to test and AI agents can understand exactly how to interact with your counter.

---

## 5. How Data and Logic Flow (5 minutes)

In MontRS, data flows in a clear loop:

1.  **UI** triggers an **Action**.
2.  **Action** validates input via **Schema** and updates the **Database**.
3.  **Router** re-triggers the **Loader**.
4.  **Loader** provides new data to the **Signals**.
5.  **Signals** update the **UI** atomically.

```text
[ UI ] --(Action)--> [ Validation ] --(Persistence)--> [ DB ]
  ^                                                      |
  |                                                      |
[ Signal Update ] <---(New Data)--- [ Loader ] <---------+
```

**Invariant**: Business logic should live in **Actions** or dedicated **Services**, never directly in the UI components.

---

## 6. Testing & Confidence (2–3 minutes)

Because MontRS is deterministic, testing is straightforward. The `TestRuntime` allows you to boot your entire application "spec" in-process.

```rust
#[tokio::test]
async fn test_increment() {
    let runtime = TestRuntime::new(MyModule);
    let result = runtime.call_action("/increment", json!({ "count": 5 })).await;
    assert_eq!(result["count"], 6);
}
```

**Why it matters**: You can test your entire backend logic without spinning up a real web server or complex infrastructure.

---

## 7. Where to Go Next

You've just scratched the surface of MontRS. Here is where to dive deeper:

-   **[Core Architecture](architecture.md)**: Understand the "Shape" of the engine.
-   **[The Golden Path](golden-path.md)**: Deep dive into idiomatic patterns.
-   **[Routing & Modules](router.md)**: Master the Loader/Action pattern.
-   **[AI Integration](ai-first.md)**: Learn how to let AI build for you.

You're now ready to build. Typically, developers start by exploring the `src/modules` folder in their new project. **Happy coding!**
