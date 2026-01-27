pub mod command;
pub mod config;
pub mod utils;
pub mod ext;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
pub enum CargoCli {
    Montrs(MontrsCli),
}

#[derive(Parser, Debug)]
#[command(name = "montrs")]
#[command(about = "MontRS Meta-framework CLI", long_about = None)]
pub struct MontrsCli {
    #[command(subcommand)]
    pub command: Commands,

    /// Build artifacts in release mode, with optimizations.
    #[arg(short, long)]
    pub release: bool,

    /// Turn on partial hot-reloading.
    #[arg(long)]
    pub hot_reload: bool,

    /// Precompress static assets with gzip and brotli.
    #[arg(short = 'P', long)]
    pub precompress: bool,

    /// Include debug information in Wasm output.
    #[arg(long)]
    pub wasm_debug: bool,

    /// Minify javascript assets.
    #[arg(long, default_value = "true")]
    pub js_minify: bool,

    /// Split WASM binary based on #[lazy] macros.
    #[arg(long)]
    pub split: bool,

    /// Only build the frontend.
    #[arg(long)]
    pub frontend_only: bool,

    /// Only build the server.
    #[arg(long, conflicts_with = "frontend_only")]
    pub server_only: bool,

    /// The features to use when compiling all targets.
    #[arg(long)]
    pub features: Vec<String>,

    /// Verbosity (none: info, errors & warnings, -v: verbose, -vv: very verbose).
    #[arg(short, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Output logs from dependencies (multiple --log accepted).
    #[arg(long)]
    pub log: Vec<String>,

    /// Use tailwind.toml to generate tailwind.config.js (Pure Rust config).
    #[arg(long)]
    pub tailwind_toml: bool,

    /// Use Tailwind v4 CSS-only configuration (No JS/TOML needed).
    #[arg(long, conflicts_with = "tailwind_toml")]
    pub tailwind_v4: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Build the project for production.
    Build,
    /// Serve the project for development with hot-reload.
    Serve,
    /// Watch for changes and rebuild.
    Watch,
    /// Run cargo tests for app, client and server.
    Test {
        /// If specified, filters tests by name.
        filter: Option<String>,

        /// Report format (human, json, junit).
        #[arg(long, default_value = "human")]
        report: String,

        /// Output file for the report (if not human).
        #[arg(long)]
        output: Option<String>,

        /// Run tests in parallel jobs.
        #[arg(short = 'j', long)]
        jobs: Option<usize>,
    },
    /// Run performance benchmarks.
    Bench {
        /// The target file to benchmark (optional).
        #[arg(index = 1)]
        target: Option<String>,

        /// Number of measurement iterations.
        #[arg(long, default_value = "100")]
        iterations: u32,

        /// Number of warm-up iterations.
        #[arg(long, default_value = "10")]
        warmup: u32,

        /// Benchmark timeout in seconds.
        #[arg(long)]
        timeout: Option<u64>,

        /// Filter benchmarks by name.
        #[arg(short, long)]
        filter: Option<String>,

        /// Export report to JSON file.
        #[arg(long)]
        json_output: Option<String>,

        /// Simple/Native mode: Benchmarks a file/binary directly without project overhead.
        #[arg(long)]
        simple: bool,

        /// Generate weight file from benchmark results (Substrate-style).
        #[arg(long)]
        generate_weights: Option<String>,
    },
    /// Start the server and end-2-end tests.
    #[command(name = "e2e")]
    E2e {
        /// Run browsers in headless mode.
        #[arg(long)]
        headless: bool,

        /// Keep the server running after tests complete.
        #[arg(long)]
        keep_alive: bool,

        /// Specify browser to use (chromium, firefox, webkit).
        #[arg(long)]
        browser: Option<String>,
    },
    /// Create a new project from a template.
    New {
        /// Name of the project.
        name: String,
        /// Template to use.
        #[arg(short, long, default_value = "default")]
        template: String,
    },
    /// Run custom tasks defined in montrs.toml.
    Run {
        /// Name of the task to run.
        task: String,
    },
    /// List available tasks.
    Tasks,
    /// Generate shell completions.
    Completions {
        /// Shell to generate completions for.
        shell: clap_complete::Shell,
    },
    /// Upgrade cargo-montrs to the latest version.
    Upgrade,
}

pub async fn run(cli: MontrsCli) -> anyhow::Result<()> {
    // Setup logger based on verbosity
    let log_level = match cli.verbose {
        0 => tracing::Level::INFO,
        1 => tracing::Level::DEBUG,
        _ => tracing::Level::TRACE,
    };
    
    // Initialize tracing if not already initialized
    let _ = tracing_subscriber::fmt()
        .with_max_level(log_level)
        .try_init();

    let mut config = config::MontrsConfig::load()?;
    config.project.verbose = cli.verbose;
    config.project.log = cli.log.clone();
    config.project.release = cli.release;
    config.project.hot_reload = cli.hot_reload;
    config.project.precompress = cli.precompress;
    config.project.wasm_debug = cli.wasm_debug;
    config.project.js_minify = cli.js_minify;
    config.project.split = cli.split;
    config.project.frontend_only = cli.frontend_only;
    config.project.server_only = cli.server_only;
    config.project.features = cli.features.clone();

    if cli.tailwind_toml {
        config.project.tailwind_style = Some(config::TailwindStyle::Toml);
    } else if cli.tailwind_v4 {
        config.project.tailwind_style = Some(config::TailwindStyle::V4);
    }

    match cli.command {
        Commands::Build => command::build::run().await,
        Commands::Serve => command::serve::run().await,
        Commands::Watch => command::watch::run().await,
        Commands::Test {
            filter,
            report,
            output,
            jobs,
        } => command::test::run(filter, report, output, jobs).await,
        Commands::Bench {
            target,
            iterations,
            warmup,
            timeout,
            filter,
            json_output,
            simple,
            generate_weights,
        } => command::bench::run(target, iterations, warmup, timeout, filter, json_output, simple, generate_weights).await,
        Commands::E2e { headless, keep_alive, browser } => command::e2e::run(headless, keep_alive, browser).await,
        Commands::New { name, template } => command::new::run(name, template).await,
        Commands::Run { task } => command::run::run(task).await,
        Commands::Tasks => command::run::list().await,
        Commands::Completions { shell } => {
            use clap::CommandFactory;
            let mut cmd = MontrsCli::command();
            clap_complete::generate(shell, &mut cmd, "cargo-montrs", &mut std::io::stdout());
            Ok(())
        }
        Commands::Upgrade => command::upgrade::run().await,
    }
}
