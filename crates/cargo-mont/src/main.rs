use cargo_mont::{Commands, run};
use clap::Parser;

#[derive(Parser)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
enum CargoCli {
    Mont(MontCli),
}

#[derive(Parser)]
#[command(name = "mont")]
#[command(about = "MontRS Meta-framework CLI", long_about = None)]
struct MontCli {
    #[command(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // When run as a cargo subcommand, the first argument is "mont"
    let args: Vec<String> = std::env::args().collect();
    let cli = if args.get(1).map(|s| s.as_str()) == Some("mont") {
        let CargoCli::Mont(mont) = CargoCli::parse();
        mont
    } else {
        MontCli::parse()
    };

    if let Err(e) = run(cli.command).await {
        eprintln!("{} Error: {:?}", style("✘").red().bold(), e);
        std::process::exit(1);
    }
}

use console::style;
