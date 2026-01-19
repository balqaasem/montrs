use crate::ext::exe_command;
use std::process::Command;

pub async fn run() -> anyhow::Result<()> {
    println!("Building MontRS project via trunk...");

    // 1. Compile styles (if needed)
    // For now, let trunk handle it via its config or manual trigger

    // 2. Build Frontend via Trunk
    let mut trunk_cmd = Command::new("trunk");
    trunk_cmd.arg("build").arg("--release");

    exe_command(&mut trunk_cmd)?;

    // 3. Build Server via Cargo
    println!("Building server via cargo...");
    let mut cargo_cmd = Command::new("cargo");
    cargo_cmd.arg("build").arg("--release");

    exe_command(&mut cargo_cmd)?;

    println!("Build complete!");
    Ok(())
}
