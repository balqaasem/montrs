//! montrs-cli: The official scaffolding tool for MontRS.
//! This tool helps developers initialize new projects with a workspace-first
//! architecture and integration with common developer tools.

use clap::{Parser, Subcommand};
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::PathBuf;

/// Command-line arguments for the CLI tool.
#[derive(Parser)]
#[command(name = "create-mont-app")]
#[command(about = "Scaffold a new MontRS application", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new project from a template.
    New {
        /// Name of the project to be created as a subdirectory.
        name: String,
        /// Template to use for scaffolding (default is currently the only supported template).
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

/// Core logic for provisioning a new MontRS project.
/// Handles directory creation and template files generation.
async fn provision_project(name: &str, _template: &str) -> anyhow::Result<()> {
    println!(
        "{} Creating new project: {}",
        style("🚀").bold(),
        style(name).cyan().bold()
    );

    let base_path = PathBuf::from(name);
    if base_path.exists() {
        return Err(anyhow::anyhow!("Directory {} already exists", name));
    }

    // Set up a progress bar for visual feedback during project creation.
    let pb = ProgressBar::new(4);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}",
            )?
            .progress_chars("#>-"),
    );

    // 1. Initialize the directory structure (workspace-first).
    pb.set_message("Creating directory structure...");
    fs::create_dir_all(base_path.join("crates"))?;
    fs::create_dir_all(base_path.join("app/src"))?;
    fs::create_dir_all(base_path.join("docs"))?;
    pb.inc(1);

    // 2. Write the root workspace Cargo.toml.
    pb.set_message("Writing workspace config...");
    let cargo_toml = r#"[workspace]
resolver = "2"
members = ["app"]

[workspace.package]
version = "0.1.0"
edition = "2024"
"#;
    fs::write(base_path.join("Cargo.toml"), cargo_toml)?;
    pb.inc(1);

    // 3. Generate the application core skeleton (Leptos-ready).
    pb.set_message("Generating Leptos application...");
    let app_cargo = r#"[package]
name = "app"
version = "0.1.0"
edition = "2024"

[dependencies]
montrs-core = { git = "https://github.com/afsall-labs/mont-rs.git" }
leptos = { version = "0.8", features = ["hydrate", "ssr"] }
"#;
    fs::write(base_path.join("app/Cargo.toml"), app_cargo)?;

    let app_main = r#"use leptos::prelude::*;
use montrs_core::{AppSpec, Target, AppConfig, EnvConfig, EnvError, FromEnv};

#[derive(Clone)]
struct MyAppConfig;
impl AppConfig for MyAppConfig {
    type Error = Box<dyn std::error::Error + Send + Sync>;
    type Env = MyEnv;
}

#[derive(Clone)]
struct MyEnv;
impl EnvConfig for MyEnv {
    fn get<T: FromEnv>(&self, _key: &str) -> Result<T, EnvError> {
        Err(EnvError::MissingKey(_key.to_string()))
    }
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <main class="flex flex-col items-center justify-center min-h-screen bg-slate-900 text-white">
            <h1 class="text-4xl font-bold mb-4">"Built with MontRS & Leptos"</h1>
            <button
                class="px-6 py-2 bg-blue-600 rounded-lg hover:bg-blue-500 transition-colors"
                on:click=move |_| set_count.update(|n| *n += 1)
            >
                "Count: " {count}
            </button>
        </main>
    }
}

fn main() {
    let spec = AppSpec::new(MyAppConfig, MyEnv)
        .with_target(Target::Wasm);
    
    spec.mount(|| view! { <App /> });
}
"#;
    fs::write(base_path.join("app/src/main.rs"), app_main)?;

    let index_html = r#"<!DOCTYPE html>
<html>
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>MontRS App</title>
    <script src="https://cdn.tailwindcss.com"></script>
  </head>
  <body></body>
</html>
"#;
    fs::write(base_path.join("index.html"), index_html)?;
    pb.inc(1);

    // 4. Add developer ergonomics (Makefiles, Trunk, etc.).
    pb.set_message("Adding developer ergonomics...");
    let makefile = r#"[tasks.dev]
command = "trunk"
args = ["serve"]

[tasks.test]
command = "cargo"
args = ["test", "--workspace"]

[tasks.build]
command = "trunk"
args = ["build", "--release"]
"#;
    fs::write(base_path.join("Makefile.toml"), makefile)?;

    let trunk_toml = r#"[build]
target = "index.html"
dist = "dist"

[serve]
port = 8080
"#;
    fs::write(base_path.join("trunk.toml"), trunk_toml)?;

    let gitignore = r#"/target
/dist
**/*.rs.bk
Cargo.lock
.env
"#;
    fs::write(base_path.join(".gitignore"), gitignore)?;
    pb.inc(1);

    // 5. Finalize setup.
    pb.set_message("Finalizing...");
    pb.inc(1);
    pb.finish_with_message("Done!");

    println!(
        "\n{} Project {} created successfully!",
        style("✨").green().bold(),
        style(name).cyan().bold()
    );
    println!("Next steps:\n  cd {}\n  cargo run -p app", name);

    Ok(())
}
