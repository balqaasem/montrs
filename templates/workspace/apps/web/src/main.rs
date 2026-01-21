use leptos::prelude::*;
use tailwind_fuse::*;
use ui::Button;

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
    leptos::mount::mount_to_body(App);
}
