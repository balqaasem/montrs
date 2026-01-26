use crate::config::MontConfig;
use anyhow::{Result, anyhow};
use clap::Parser;

pub async fn run_cargo_leptos(cmd: &str, args: &[String], config: &MontConfig) -> Result<()> {
    // Build arguments for cargo-leptos
    let mut args_list = vec!["cargo-leptos".to_string(), cmd.to_string()];

    if config.project.release {
        args_list.push("--release".to_string());
    }
    if config.project.precompress {
        args_list.push("--precompress".to_string());
    }
    if config.project.hot_reload {
        args_list.push("--hot-reload".to_string());
    }
    if config.project.wasm_debug {
        args_list.push("--wasm-debug".to_string());
    }
    if config.project.js_minify {
        args_list.push("--js-minify".to_string());
    } else {
        args_list.push("--js-minify=false".to_string());
    }
    if config.project.split {
        args_list.push("--split".to_string());
    }
    if config.project.frontend_only {
        args_list.push("--frontend-only".to_string());
    }
    if config.project.server_only {
        args_list.push("--server-only".to_string());
    }

    for feature in &config.project.features {
        args_list.push("--features".to_string());
        args_list.push(feature.clone());
    }

    for _ in 0..config.project.verbose {
        args_list.push("-v".to_string());
    }

    // Add trailing arguments for serve/watch
    if !args.is_empty() {
        args_list.push("--".to_string());
        for arg in args {
            args_list.push(arg.clone());
        }
    }

    let cli = cargo_leptos::config::Cli::try_parse_from(args_list)
        .map_err(|e| anyhow!("Failed to parse cargo-leptos arguments: {}", e))?;

    cargo_leptos::run(cli).await.map_err(|e| anyhow!("{:?}", e))
}
