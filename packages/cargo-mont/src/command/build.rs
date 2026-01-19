use crate::ext::exe_command;
use std::process::Command;

pub async fn run() -> anyhow::Result<()> {
    let config = crate::config::MontConfig::load()?;
    println!("Building {}...", config.project.name);

    // 1. Compile styles (if needed)
    // For now, let the build system handle it via config

    // 2. Build Frontend
    let mut front_cmd = Command::new("trunk");
    front_cmd.arg("build").arg("--release")
        .arg("--config").arg("mont.toml")
        .arg("--target").arg(&config.build.target)
        .arg("--dist").arg(&config.build.dist);

    if let Some(assets) = &config.build.assets_dir {
        front_cmd.arg("--assets-dir").arg(assets);
    }

    exe_command(&mut front_cmd)?;

    // 3. Build Server via Cargo
    println!("Building server via cargo...");
    let mut cargo_cmd = Command::new("cargo");
    cargo_cmd.arg("build").arg("--release");

    exe_command(&mut cargo_cmd)?;

    println!("Build complete!");
    Ok(())
}
