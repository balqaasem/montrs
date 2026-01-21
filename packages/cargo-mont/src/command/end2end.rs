use crate::config::MontConfig;

pub async fn run() -> anyhow::Result<()> {
    let config = MontConfig::load()?;
    let leptos_config = config.to_leptos_config(false)?;

    cargo_leptos::command::end2end_all(&leptos_config).await
}
