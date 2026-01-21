use crate::config::MontConfig;

pub async fn run() -> anyhow::Result<()> {
    let config = MontConfig::load()?;
    let leptos_config = config.to_leptos_config(false)?;
    let project = leptos_config.current_project()?;

    // Initialize the ctrl-c monitor as cargo-leptos expects it
    let _monitor = cargo_leptos::signal::Interrupt::run_ctrl_c_monitor();

    cargo_leptos::command::serve(&project).await
}
