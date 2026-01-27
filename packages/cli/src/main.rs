use cargo_montrs::{CargoCli, MontrsCli, run};
use clap::Parser;

#[tokio::main]
async fn main() {
    println!("Cargo-MontRS starting...");
    // When run as a cargo subcommand, the first argument is "montrs"
    let args: Vec<String> = std::env::args().collect();
    let cli = if args.get(1).map(|s| s.as_str()) == Some("montrs") {
        let CargoCli::Montrs(montrs) = CargoCli::parse();
        montrs
    } else {
        MontrsCli::parse()
    };

    if let Err(e) = run(cli).await {
        eprintln!("{} Error: {:?}", style("✘").red().bold(), e);
        std::process::exit(1);
    }
}

use console::style;
