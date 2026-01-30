//! todo-example: A comprehensive example demonstrating MontRS features.
//! This application integrates signals, schema validation, and the ORM layer
//! to build a simple but functional Todo management system.

use leptos::prelude::*;
use montrs_core::{
    AppConfig, AppSpec, Plate, PlateContext, Route, RouteAction, RouteContext, RouteError,
    RouteLoader, RouteParams, RouteView, Router, Target,
};
use montrs_orm::{DbBackend, FromRow, SqliteBackend};
use montrs_schema::Schema;
use serde::{Deserialize, Serialize};

// 1. Define the Schema for creating a Todo.
#[derive(Debug, Clone, Serialize, Deserialize, Schema)]
pub struct CreateTodo {
    #[schema(min_len = 3)]
    pub title: String,
}

// Data model representing a Todo item in the database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

impl FromRow for Todo {
    fn from_row_sqlite(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            title: row.get(1)?,
            completed: row.get(2)?,
        })
    }

    fn from_row_postgres(_row: &tokio_postgres::Row) -> Result<Self, montrs_orm::DbError> {
        Err(montrs_orm::DbError::Query(
            "Postgres not fully implemented in example".to_string(),
        ))
    }
}

// Custom error type for our application.
#[derive(Debug, thiserror::Error)]
pub enum MyError {
    #[error("Database error: {0}")]
    Db(#[from] montrs_orm::DbError),
    #[error("Generic error: {0}")]
    Generic(String),
}

// 2. Define the Application Configuration.
#[derive(Clone)]
pub struct MyConfig {
    pub db: SqliteBackend,
}

impl AppConfig for MyConfig {
    type Error = MyError;
    type Env = MyEnv;
}

#[derive(Clone)]
pub struct MyEnv;

impl montrs_core::EnvConfig for MyEnv {
    fn get_var(&self, key: &str) -> Result<String, montrs_core::EnvError> {
        Err(montrs_core::EnvError::MissingKey(key.to_string()))
    }
}

// 3. Define the Route for the Todo Application.
#[derive(Serialize, Deserialize)]
pub struct TodoParams {}
impl RouteParams for TodoParams {}

pub struct TodoLoader;
#[async_trait::async_trait]
impl RouteLoader<TodoParams, MyConfig> for TodoLoader {
    type Output = Vec<Todo>;
    async fn load(
        &self,
        _ctx: RouteContext<'_, MyConfig>,
        _params: TodoParams,
    ) -> Result<Self::Output, RouteError> {
        // In a real app, we'd fetch from the database.
        Ok(vec![])
    }
}

pub struct TodoAction;
#[async_trait::async_trait]
impl RouteAction<TodoParams, MyConfig> for TodoAction {
    type Input = CreateTodo;
    type Output = Todo;
    async fn act(
        &self,
        _ctx: RouteContext<'_, MyConfig>,
        _params: TodoParams,
        _input: Self::Input,
    ) -> Result<Self::Output, RouteError> {
        // In a real app, we'd save to the database.
        Ok(Todo {
            id: 1,
            title: "New Todo".to_string(),
            completed: false,
        })
    }
}

pub struct TodoViewImpl;
impl RouteView for TodoViewImpl {
    fn render(&self) -> impl IntoView {
        view! { <TodoApp /> }
    }
}

pub struct TodoRoute;
impl Route<MyConfig> for TodoRoute {
    type Params = TodoParams;
    type Loader = TodoLoader;
    type Action = TodoAction;
    type View = TodoViewImpl;

    fn path() -> &'static str {
        "/"
    }
    fn loader(&self) -> Self::Loader {
        TodoLoader
    }
    fn action(&self) -> Self::Action {
        TodoAction
    }
    fn view(&self) -> Self::View {
        TodoViewImpl
    }
}

// 4. Define a Plate for Todo logic.
pub struct TodoPlate;

#[async_trait::async_trait]
impl Plate<MyConfig> for TodoPlate {
    fn name(&self) -> &'static str {
        "todo"
    }

    async fn init(
        &self,
        _ctx: &mut PlateContext<MyConfig>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("TodoPlate initialized");
        Ok(())
    }

    fn register_routes(&self, router: &mut Router<MyConfig>) {
        router.register(TodoRoute);
        println!("Routes registered for TodoPlate");
    }
}

#[component]
fn TodoApp() -> impl IntoView {
    let (count, set_count) = signal(0);
    // Access the provided application config context.
    let _config = use_context::<MyConfig>();

    view! {
        <div class="p-8 max-w-md mx-auto bg-white rounded-xl shadow-md overflow-hidden md:max-w-2xl mt-10">
            <h1 class="text-2xl font-bold text-gray-900 mb-4">"MontRS Todo (Leptos 0.8)"</h1>
            <p class="text-gray-500 mb-4">"This example uses Leptos's high-performance fine-grained reactivity."</p>
            {move || {
                if _config.is_some() {
                    view! { <p class="text-green-600 mb-4">"AppConfig context is available!"</p> }.into_any()
                } else {
                    view! { <p class="text-red-600 mb-4">"AppConfig context missing."</p> }.into_any()
                }
            }}
            <button
                class="bg-indigo-600 text-white px-4 py-2 rounded-lg hover:bg-indigo-500 transition-colors"
                on:click=move |_| set_count.update(|n| *n += 1)
            >
                "Clicked " {count} " times"
            </button>
        </div>
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 4. Bootstrap the Application Specification (AppSpec).
    let db = SqliteBackend::new(":memory:")?;
    let config = MyConfig { db };
    let env = MyEnv;

    let spec = AppSpec::new(config, env)
        .with_target(Target::Wasm)
        .with_plate(Box::new(TodoPlate));

    println!("Bootstrapping MontRS application...");

    // 5. Demonstrate Schema Validation (Meta-framework feature).
    let valid_todo = CreateTodo {
        title: "Build with Leptos".to_string(),
    };
    println!("Validation check: {:?}", valid_todo.validate());

    // 6. Demonstrate ORM operations (Meta-framework feature).
    spec.config
        .db
        .execute(
            "CREATE TABLE IF NOT EXISTS todos (id INTEGER PRIMARY KEY, title TEXT, completed BOOLEAN)",
            &[],
        )
        .await?;

    // 7. Mount the Leptos application.
    // In a WASM environment, this would start the reactivity loop.
    println!("Mounting Leptos application...");
    spec.mount(|| view! { <TodoApp /> });

    Ok(())
}
