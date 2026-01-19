pub mod command;
pub mod compile;
pub mod config;
pub mod ext;
pub mod service;

use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Build the project for production.
    Build,
    /// Serve the project for development with hot-reload.
    Serve,
    /// Create a new project from a template.
    New {
        /// Name of the project.
        name: String,
        /// Template to use.
        #[arg(short, long, default_value = "default")]
        template: String,
    },
}

pub async fn run(command: Commands) -> anyhow::Result<()> {
    match command {
        Commands::Build => command::build::run().await,
        Commands::Serve => command::serve::run().await,
        Commands::New { name, template } => command::new::run(name, template).await,
    }
}
