use crate::config::MontConfig;

pub async fn run() -> anyhow::Result<()> {
    let mut config = MontConfig::load()?;

    // Handle tailwind.toml
    if let Ok(Some(js_path)) = crate::config::tailwind::ensure_tailwind_config(
        std::path::Path::new("."),
        config.project.tailwind_style.unwrap_or_default(),
    ) {
        if config.build.tailwind_config_file.is_none() {
            config.build.tailwind_config_file = Some(js_path.to_string_lossy().into_owned());
        }
    }

    let leptos_config = config.to_leptos_config(false)?;
    let project = leptos_config.current_project()?;

    // Initialize the ctrl-c monitor as cargo-leptos expects it
    let _monitor = cargo_leptos::signal::Interrupt::run_ctrl_c_monitor();

    cargo_leptos::command::serve(&project).await
}
