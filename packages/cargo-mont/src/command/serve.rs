use crate::ext::exe_command;
use std::process::Command;
use tokio::signal;

pub async fn run() -> anyhow::Result<()> {
    let config = crate::config::MontConfig::load()?;
    println!("Starting {} development services...", config.project.name);

    // 1. Start Frontend Service (hot-reload)
    let config_clone = config.clone();
    let _frontend = tokio::spawn(async move {
        println!("Starting frontend watcher...");
        let mut front_cmd = Command::new("trunk");
        front_cmd
            .arg("serve")
            .arg("--config")
            .arg("mont.toml")
            .arg("--port")
            .arg(config_clone.serve.port.to_string())
            .arg("--address")
            .arg(&config_clone.serve.addr)
            .arg("--target")
            .arg(&config_clone.build.target);

        if let Err(e) = exe_command(&mut front_cmd) {
            eprintln!("Front-end service failed: {:?}", e);
        }
    });

    // 2. Start Server Watcher
    let config_clone2 = config.clone();
    let _server = tokio::spawn(async move {
        println!("Starting server-side runner (cargo run)...");
        let mut cmd = Command::new("cargo");
        cmd.arg("run");

        // Inject Leptos-style env vars
        cmd.env("LEPTOS_SITE_ROOT", &config_clone2.build.site_root);
        cmd.env("LEPTOS_SITE_PKG_NAME", &config_clone2.build.site_pkg_name);
        cmd.env(
            "LEPTOS_SITE_ADDR",
            format!("{}:{}", config_clone2.serve.addr, config_clone2.serve.port),
        );

        if let Err(e) = exe_command(&mut cmd) {
            eprintln!("Server service failed: {:?}", e);
        }
    });

    // Wait for shutdown signal
    signal::ctrl_c().await?;
    println!("Shutting down services...");

    Ok(())
}
