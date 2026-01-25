use cargo_mont::{CargoCli, MontCli, run};
use clap::Parser;

#[tokio::main]
async fn main() {
    println!("Cargo-mont starting...");
    // When run as a cargo subcommand, the first argument is "mont"
    let args: Vec<String> = std::env::args().collect();
    let cli = if args.get(1).map(|s| s.as_str()) == Some("mont") {
        let CargoCli::Mont(mont) = CargoCli::parse();
        mont
    } else {
        MontCli::parse()
    };

    if let Err(e) = run(cli).await {
        eprintln!("{} Error: {:?}", style("✘").red().bold(), e);
        std::process::exit(1);
    }
}

use console::style;
