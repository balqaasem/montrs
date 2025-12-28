use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;
use console::style;
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Parser)]
#[command(name = "create-mont-app")]
#[command(about = "Scaffold a new MontRS application", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new project
    New {
        /// Name of the project
        name: String,
        /// Template to use
        #[arg(short, long, default_value = "default")]
        template: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { name, template } => {
            provision_project(name, template).await?;
        }
    }

    Ok(())
}

async fn provision_project(name: &str, template: &str) -> anyhow::Result<()> {
    println!("{} Creating new project: {}", style("🚀").bold(), style(name).cyan().bold());
    
    let base_path = PathBuf::from(name);
    if base_path.exists() {
        return Err(anyhow::anyhow!("Directory {} already exists", name));
    }

    let pb = ProgressBar::new(4);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")?
        .progress_chars("#>-"));

    // 1. Create directory structure
    pb.set_message("Creating directory structure...");
    fs::create_dir_all(base_path.join("crates"))?;
    fs::create_dir_all(base_path.join("app/src"))?;
    fs::create_dir_all(base_path.join("docs"))?;
    pb.inc(1);

    // 2. Write workspace files
    pb.set_message("Writing workspace config...");
    let cargo_toml = format!(r#"[workspace]
resolver = "2"
members = ["app", "crates/*"]

[workspace.package]
version = "0.1.0"
edition = "2024"
"#);
    fs::write(base_path.join("Cargo.toml"), cargo_toml)?;
    pb.inc(1);

    // 3. Write app skeleton
    pb.set_message("Generating application core...");
    let app_cargo = r#"[package]
name = "app"
version = "0.1.0"
edition = "2024"

[dependencies]
montrs-core = { git = "https://github.com/afsall-labs/mont-rs.git" }
"#;
    fs::write(base_path.join("app/Cargo.toml"), app_cargo)?;
    fs::write(base_path.join("app/src/main.rs"), "fn main() { println!(\"Hello, MontRS!\"); }")?;
    pb.inc(1);

    // 4. Finalize
    pb.set_message("Finalizing...");
    pb.inc(1);
    pb.finish_with_message("Done!");

    println!("\n{} Project {} created successfully!", style("✨").green().bold(), style(name).cyan().bold());
    println!("Next steps:\n  cd {}\n  cargo run -p app", name);

    Ok(())
}
