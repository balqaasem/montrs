use std::process::Command;

pub async fn run() -> anyhow::Result<()> {
    println!("Upgrading cargo-montrs...");
    let status = Command::new("cargo")
        .args(["install", "cargo-montrs"])
        .status()?;

    if status.success() {
        println!("Successfully upgraded cargo-montrs!");
    } else {
        anyhow::bail!("Failed to upgrade cargo-montrs");
    }
    Ok(())
}
