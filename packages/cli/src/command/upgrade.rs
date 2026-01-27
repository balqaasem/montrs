use crate::config::{Channel, MontrsConfig};
use std::process::Command;
use colored::*;

pub async fn run() -> anyhow::Result<()> {
    let config = MontrsConfig::load().unwrap_or_default();
    let channel = config.project.channel;

    println!("Upgrading montrs on {} channel...", channel.to_string().cyan());

    let mut cmd = Command::new("cargo");
    cmd.arg("install");

    match channel {
        Channel::Stable => {
            cmd.arg("montrs");
        }
        Channel::Nightly => {
            // For nightly, we install from the git repository develop branch
            cmd.args(["--git", "https://github.com/afsall-labs/montrs", "--branch", "develop", "montrs"]);
        }
    }

    let status = cmd.status()?;

    if status.success() {
        println!("Successfully upgraded montrs to the latest {} version!", channel.to_string().cyan());
    } else {
        anyhow::bail!("Failed to upgrade montrs on {} channel", channel.to_string().red());
    }
    Ok(())
}
