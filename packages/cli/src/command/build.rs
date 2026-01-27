use crate::config::{Channel, MontrsConfig};
use crate::utils::run_cargo_leptos;
use colored::*;

pub async fn run() -> anyhow::Result<()> {
    let mut config = MontrsConfig::load()?;

    if config.project.channel == Channel::Nightly {
        println!(
            "{} Using {} features and optimizations.",
            "Nightly:".yellow().bold(),
            "experimental".italic()
        );
    }

    // Handle tailwind.toml
    if let Ok(Some(js_path)) = crate::config::tailwind::ensure_tailwind_config(
        std::path::Path::new("."),
        config.project.tailwind_style.unwrap_or_default(),
    ) {
        if config.build.tailwind_config_file.is_none() {
            config.build.tailwind_config_file = Some(js_path.to_string_lossy().into_owned());
        }
    }

    run_cargo_leptos("build", &[], &config).await
}
