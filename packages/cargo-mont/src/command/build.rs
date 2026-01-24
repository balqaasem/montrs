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

    cargo_leptos::command::build_all(&leptos_config).await
}
