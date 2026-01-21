use leptos::prelude::*;
use montrs_core::{AppSpec, Target, AppConfig, EnvConfig, EnvError, FromEnv};
use ui::Button;

#[derive(Clone)]
struct MyAppConfig;
impl AppConfig for MyAppConfig {
    type Error = Box<dyn std::error::Error + Send + Sync>;
    type Env = MyEnv;
}

#[derive(Clone)]
struct MyEnv;
impl EnvConfig for MyEnv {
    fn get<T: FromEnv>(&self, _key: &str) -> Result<T, EnvError> {
        Err(EnvError::MissingKey(_key.to_string()))
    }
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <main class="flex flex-col items-center justify-center min-h-screen bg-slate-900 text-white">
            <h1 class="text-4xl font-bold mb-4">"MontRS Workspace"</h1>
            <Button on_click=move |_| set_count.update(|n| *n += 1)>
                "Count: " {count}
            </Button>
            <p class="mt-4 text-gray-400 text-sm">
                "Using shared UI from " <code class="bg-slate-800 px-1 rounded">"packages/ui"</code>
            </p>
        </main>
    }
}

fn main() {
    let spec = AppSpec::new(MyAppConfig, MyEnv)
        .with_target(Target::Wasm);
    
    spec.mount(|| view! { <App /> });
}
