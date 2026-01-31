# Routing in MontRS: The Unified Route Trait

MontRS uses a "Unified Route" model that combines parameters, data fetching (`RouteLoader`), state changes (`RouteAction`), and visual representation (`RouteView`) into a single, type-safe unit.

## 🛣️ The Unified Route Trait

Instead of disparate loaders and actions, MontRS defines a route as a struct implementing the `Route` trait. This provides a single source of truth for everything related to a specific URL path.

```rust
pub trait Route<C: AppConfig>: Send + Sync + 'static {
    type Params: RouteParams;
    type Loader: RouteLoader<Self::Params, C>;
    type Action: RouteAction<Self::Params, C>;
    type View: RouteView;

    fn path() -> &'static str;
    fn loader(&self) -> Self::Loader;
    fn action(&self) -> Self::Action;
    fn view(&self) -> Self::View;
}

### 🏗️ Creating a Route via CLI

The recommended way to add a route to a plate is using the CLI:

```bash
montrs generate route <path> --plate <plate_name>
```

This generates a unified `Route` struct bundling `Params`, `Loader`, `Action`, and `View` into a single file, ensuring consistency and reducing boilerplate.

### 🏗️ Manual Route Implementation

If you're not using the CLI, you must implement the `Route` trait:

```rust
pub struct UserRoute;

impl Route<AppConfig> for UserRoute {
    type Params = UserParams;
    type Loader = UserLoader;
    type Action = UserAction;
    type View = UserView;

    fn path() -> &'static str { "/users/:id" }
    fn loader(&self) -> Self::Loader { UserLoader }
    fn action(&self) -> Self::Action { UserAction }
    fn view(&self) -> Self::View { UserView }
}
```

## 📥 RouteParams: Type-Safe Parameters

Every route defines its own parameter structure, which is automatically deserialized from the URL.

```rust
#[derive(Serialize, Deserialize)]
pub struct UserParams {
    pub id: String,
}
impl RouteParams for UserParams {}
```

## 📥 RouteLoader: Fetching Data

A `RouteLoader` is responsible for fetching the data needed for a route. It is read-only and idempotent.

```rust
#[async_trait]
impl RouteLoader<UserParams, MyConfig> for UserLoader {
    type Output = User;
    async fn load(&self, ctx: RouteContext<'_, MyConfig>, params: UserParams) -> Result<Self::Output, RouteError> {
        let user = ctx.db().get_user(&params.id).await?;
        Ok(user)
    }
}
```

## 📤 RouteAction: State Changes

A `RouteAction` handles mutations (POST, PUT, DELETE). It explicitly defines its input and output types.

```rust
#[async_trait]
impl RouteAction<UserParams, MyConfig> for UpdateUserAction {
    type Input = UpdateUserInput;
    type Output = User;
    async fn act(&self, ctx: RouteContext<'_, MyConfig>, params: UserParams, input: Self::Input) -> Result<Self::Output, RouteError> {
        let user = ctx.db().update_user(&params.id, input).await?;
        Ok(user)
    }
}
```

## 🖼️ RouteView: Visual Representation

The `RouteView` defines how the route is rendered, typically using Leptos components.

```rust
impl RouteView for UserView {
    fn render(&self) -> impl IntoView {
        view! { <UserPage /> }
    }
}
```

## 🧩 Registration in Plates

Routes are registered within a `Plate` using the `Router`.

```rust
impl Plate<MyConfig> for UserPlate {
    fn name(&self) -> &'static str { "user" }
    
    fn dependencies(&self) -> Vec<&'static str> {
        vec!["auth"] // Depends on auth plate
    }

    fn register_routes(&self, router: &mut Router<MyConfig>) {
        router.register(UserDetailRoute);
        router.register(UserListRoute);
    }
}
```

## 🔄 The Request Lifecycle

1.  **Match**: The `Router` finds the matching route based on the URL path.
2.  **Parse**: `RouteParams` are extracted and validated from the URL.
3.  **Execute**: 
    - For GET: The `RouteLoader` is called.
    - For Mutations: The `RouteAction` is called with the provided input.
4.  **Render**: The `RouteView` is used to render the final UI (if applicable).

## 🤖 Agent-First Routing

The unified `Route` trait is designed specifically for agent discoverability. Because all parts of a route are linked through a single trait, an agent can:
1.  **See the path** and parameters required.
2.  **Understand the data** being fetched (Loader Output).
3.  **Identify valid operations** (Action Input/Output).
4.  **Explore the UI** (View).

This metadata is automatically exposed through the `AppSpec`, allowing agents to navigate and interact with your application with high confidence.
