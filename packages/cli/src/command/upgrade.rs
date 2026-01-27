use std::process::Command;

pub async fn run() -> anyhow::Result<()> {
    println!("Upgrading montrs...");
    let status = Command::new("cargo")
        .args(["install", "montrs"])
        .status()?;

    if status.success() {
        println!("Successfully upgraded montrs!");
    } else {
        anyhow::bail!("Failed to upgrade montrs");
    }
    Ok(())
}
