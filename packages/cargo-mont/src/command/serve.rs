use crate::ext::exe_command;
use std::process::Command;
use tokio::signal;

pub async fn run() -> anyhow::Result<()> {
    println!("Starting MontRS development services...");

    // 1. Start Trunk Serve for frontend (hot-reload)
    let _frontend = tokio::spawn(async move {
        println!("Starting frontend watcher (trunk)...");
        let mut cmd = Command::new("trunk");
        cmd.arg("serve");
        if let Err(e) = exe_command(&mut cmd) {
            eprintln!("Front-end service failed: {:?}", e);
        }
    });

    // 2. Start Server Watcher
    // For now, we'll just run cargo run -- but in a real tool we'd use notify
    let _server = tokio::spawn(async move {
        println!("Starting server-side runner (cargo run)...");
        let mut cmd = Command::new("cargo");
        cmd.arg("run");
        if let Err(e) = exe_command(&mut cmd) {
            eprintln!("Server service failed: {:?}", e);
        }
    });

    // Wait for shutdown signal
    signal::ctrl_c().await?;
    println!("Shutting down services...");

    Ok(())
}
