use leptos::prelude::*;
use montrs_core::{AppSpec, Target, AppConfig, EnvConfig, EnvError, FromEnv};
use tailwind_fuse::*;

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

#[derive(TwClass)]
#[tw(class = "px-6 py-2 rounded-lg transition-colors")]
struct CounterBtn {
    variant: BtnVariant,
}

#[derive(TwVariant)]
enum BtnVariant {
    #[tw(default, class = "bg-blue-600 hover:bg-blue-500 text-white")]
    Primary,
    #[tw(class = "bg-gray-600 hover:bg-gray-500 text-white")]
    Secondary,
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let btn_class = CounterBtn { variant: BtnVariant::Primary };

    view! {
        <main class="flex flex-col items-center justify-center min-h-screen bg-slate-900 text-white">
            <h1 class="text-4xl font-bold mb-4">"Built with MontRS & Leptos"</h1>
            <button
                class=btn_class.to_class()
                on:click=move |_| set_count.update(|n| *n += 1)
            >
                "Count: " {count}
            </button>
            <p class="mt-4 text-gray-400 text-sm">
                "Powered by " <code class="bg-slate-800 px-1 rounded">"tailwind-fuse"</code>
            </p>
        </main>
    }
}

fn main() {
    let spec = AppSpec::new(MyAppConfig, MyEnv)
        .with_target(Target::Wasm);
    
    spec.mount(|| view! { <App /> });
}
