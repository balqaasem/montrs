use std::process::Command;

pub async fn run() -> anyhow::Result<()> {
    println!("Upgrading cargo-mont...");
    let status = Command::new("cargo")
        .args(["install", "cargo-mont"])
        .status()?;

    if status.success() {
        println!("Successfully upgraded cargo-mont!");
    } else {
        anyhow::bail!("Failed to upgrade cargo-mont");
    }
    Ok(())
}
