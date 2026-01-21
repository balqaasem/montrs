use crate::config::MontConfig;

pub async fn run() -> anyhow::Result<()> {
    let config = MontConfig::load()?;
    let leptos_config = config.to_leptos_config(false)?;

    let test_opts = cargo_leptos::config::TestSpecificOpts::default();
    cargo_leptos::command::test_all(&leptos_config, &test_opts).await
}
