use crate::config::{Channel, MontrsConfig};
use anyhow::{Context, Result};
use colored::*;

/// Runs the channel management command.
pub async fn run(name: Option<String>) -> Result<()> {
    let mut config = MontrsConfig::load()?;

    match name {
        Some(channel_name) => {
            let new_channel = match channel_name.to_lowercase().as_str() {
                "stable" => Channel::Stable,
                "nightly" => Channel::Nightly,
                _ => {
                    anyhow::bail!(
                        "Invalid channel: {}. Supported channels are 'stable' and 'nightly'.",
                        channel_name.red()
                    );
                }
            };

            if config.project.channel == new_channel {
                println!("Project is already on the {} channel.", new_channel.to_string().cyan());
                return Ok(());
            }

            config.project.channel = new_channel;
            config.save().context("Failed to save configuration after channel switch")?;

            println!(
                "{} Switched to {} channel.",
                "Success:".green().bold(),
                new_channel.to_string().cyan().bold()
            );
        }
        None => {
            println!(
                "Current channel: {}",
                config.project.channel.to_string().cyan().bold()
            );
            println!("\nAvailable channels:");
            println!("  - {} (default, production-ready)", "stable".green());
            println!("  - {} (experimental features, frequent updates)", "nightly".yellow());
            println!("\nUse `montrs channel <name>` to switch.");
        }
    }

    Ok(())
}
