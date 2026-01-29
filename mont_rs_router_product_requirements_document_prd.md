# MontRS Router – Product Requirements Document (PRD)

**Package:** `montrs-core`

**Subsystem:** Router

**Status:** Design-approved, implementation-targeted

**Target Version:** v0.1.0

**License:** MIT

**Applies To:** Web, Mobile, Desktop

---

## 1. Executive Summary

The MontRS Router is a **core, first-class subsystem** of `montrs-core`. It defines how applications declare navigation structure, how data is loaded and mutated at route boundaries, and how rendering is coordinated across **web, mobile, and desktop targets**.

This router is not a port of an existing framework. Instead, it is a **purpose-built system** that integrates the strongest ideas from Remix, React Router, Leptos Router, Dioxus Router, TanStack Router, and platform-native navigation systems—while remaining faithful to MontRS’s philosophy:

- Deterministic execution
- Explicit data boundaries
- Trait-driven composition
- Target-agnostic core logic
- Strong typing and static introspection

The router is designed to be:

- **Developer-friendly** (predictable, discoverable, ergonomic)
- **AI-friendly** (static route graph, typed metadata, machine-readable)
- **Integration-friendly** (clean boundaries with modules, AppSpec, ORM)
- **Cross-platform** (single route graph, multiple adapters)

---

## 2. Design Goals

### 2.1 Primary Goals

1. Provide a **single routing model** across all MontRS targets
2. Make **data loading and mutation first-class route concerns**
3. Enable **hybrid SSR + CSR by default** on web
4. Ensure routing is **fully deterministic and testable**
5. Support **nested routes and layouts** without runtime ambiguity
6. Produce a **static route graph** usable by tooling and AI

### 2.2 Non-Goals

The MontRS Router explicitly does **not**:

- Provide multiple routing paradigms
- Depend on DOM APIs
- Use runtime string-based route resolution
- Hide side effects behind hooks
- Allow runtime mutation of the route tree

---

## 3. Conceptual Model

### 3.1 Route as a Resource Boundary

In MontRS, a route is not merely a URL pattern. It is a **resource boundary** that defines:

- What data is read (loader)
- What data may be mutated (action)
- What parameters are required
- What UI is rendered
- What errors are handled locally

This model is directly inspired by Remix but adapted to Rust’s type system and MontRS’s deterministic runtime.

---

### 3.2 Navigation as Intent

Navigation in MontRS is **intent-based**, not string-based.

Developers navigate to **typed routes**, not URLs:

```rust
navigate(UserRoute { id: user_id });
```

The router resolves the intent using a platform-specific adapter:

- Web → URL change
- Mobile → navigation stack
- Desktop → view replacement

Routes themselves are **agnostic** to the transport.

---

### 3.3 Static Route Graph

At compile time, MontRS builds a **static route graph** from route declarations.

This graph is:

- Typed
- Deterministic
- Immutable at runtime

It is used for:

- Validation
- SSR planning
- Code generation
- AI tooling
- TestRuntime execution

---

## 4. Core Components

### 4.1 Route Trait

```rust
pub trait Route {
    type Params: RouteParams;
    type Loader: RouteLoader<Self::Params>;
    type Action: RouteAction<Self::Params>;
    type View: RouteView;
}
```

Each route defines its full behavior declaratively via associated types.

---

### 4.2 Loader

Loaders are **pure read operations**.

```rust
pub trait RouteLoader<P> {
    type Output;
    fn load(ctx: RouteContext, params: P) -> Result<Self::Output, RouteError>;
}
```

Rules:

- Loaders must not mutate state
- Loaders may access modules via context
- Loaders are deterministic given inputs

---

### 4.3 Action

Actions are **explicit write operations**.

```rust
pub trait RouteAction<P> {
    fn act(ctx: RouteContext, params: P, input: ActionInput) -> Result<(), RouteError>;
}
```

Rules:

- All mutations happen in actions
- Actions integrate with ORM and modules
- Actions are auditable and testable

---

## 5. Route Declaration Syntax

### 5.1 Macro-Based Declaration

Routes are declared using macros for clarity and static analysis.

```rust
#[route("/users/:id")]
pub mod user {
    #[loader]
    pub fn load(id: UserId) -> Result<User, RouteError> { ... }

    #[action]
    pub fn update(input: UpdateUser) -> Result<(), RouteError> { ... }

    pub fn view() -> impl View { ... }
}
```

This syntax:

- Avoids runtime configuration
- Enables static introspection
- Is AI- and tooling-friendly

---

### 5.2 File Layout

Routes follow a **file-based hierarchy** that mirrors the route tree.

```
routes/
├─ mod.rs
├─ index.rs           // "/"
├─ users/
│  ├─ mod.rs          // "/users"
│  ├─ index.rs        // "/users"
│  └─ [id].rs         // "/users/:id"
└─ admin/
   ├─ mod.rs
   └─ dashboard.rs
```

Dynamic segments use `[param].rs` syntax for familiarity.

---

## 6. Nested Routes & Layouts

Nested routes inherit:

- Layouts
- Context
- Error boundaries

Parent routes may define a layout view that wraps children.

```rust
pub fn layout(children: impl View) -> impl View { ... }
```

---

## 7. SSR & CSR Integration

### 7.1 Default Behavior

- First request: SSR
- Hydration: client attaches
- Subsequent navigation: CSR

### 7.2 Loader Execution Model

- Server executes loader on initial request
- Client reuses data on hydration
- Client executes loader on navigation

---

## 8. Cross-Platform Support

### 8.1 Web

- URL-based navigation
- History API adapter

### 8.2 Mobile

- Stack-based navigation
- No URL dependency

### 8.3 Desktop

- View-based navigation
- OS-level integration optional

---

## 9. Error Handling

Routes define local error boundaries.

```rust
pub enum RouteError {
    NotFound,
    Unauthorized,
    ValidationFailed,
    InternalError,
}
```

Errors are typed and matchable.

---

## 10. Integration with AppSpec

Routes may declare:

- Required modules
- Feature flags
- Target constraints

The router validates compatibility at startup.

---

## 11. Testing & TestRuntime

The router integrates fully with `TestRuntime`.

Developers may:

- Execute loaders directly
- Simulate navigation
- Assert rendered output

---

## 12. Tooling & AI Support

The static route graph enables:

- Route visualization
- Automated docs
- AI code generation
- Static analysis

---

## 13. Versioning & Stability

- v0.x: API evolution allowed
- v1.0: API frozen

---

## 14. Security Considerations

- No implicit access to globals
- Explicit data boundaries
- Typed params prevent injection

---

## 15. Acceptance Criteria (v0.1)

- Typed route graph
- Nested routes
- Loaders & actions
- SSR + CSR web support
- Cross-platform navigation
- Deterministic tests

---

## 16. Strategic Value

The MontRS Router is a **foundational differentiator**. It unifies data, navigation, and rendering across platforms while remaining predictable and explicit.

This PRD defines the authoritative design for the MontRS Router in v0.1.

