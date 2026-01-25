use crate::config::MontConfig;
use anyhow::{Result, bail};
use cargo_leptos::{Cli, Commands, Opts, BinOpts};

pub async fn run_cargo_leptos(cmd: &str, args: &[String], config: &MontConfig) -> Result<()> {
    // Set Environment Variables from Config
    // Note: cargo-leptos reads LEPTOS_* env vars to override Cargo.toml config
    unsafe {
        std::env::set_var("LEPTOS_OUTPUT_NAME", &config.project.name);
        std::env::set_var(
            "LEPTOS_SITE_ADDR",
            format!("{}:{}", config.serve.addr, config.serve.port),
        );
        std::env::set_var("LEPTOS_SITE_ROOT", &config.build.site_root);
        std::env::set_var("LEPTOS_SITE_PKG_DIR", &config.build.site_pkg_name);

        if let Some(style) = &config.build.style_file {
            std::env::set_var("LEPTOS_STYLE_FILE", style);
        }
        if let Some(assets) = &config.build.assets_dir {
            std::env::set_var("LEPTOS_ASSETS_DIR", assets);
        }
        if let Some(tailwind) = &config.build.tailwind_input_file {
            std::env::set_var("LEPTOS_TAILWIND_INPUT_FILE", tailwind);
        }
        if let Some(tailwind_config) = &config.build.tailwind_config_file {
            std::env::set_var("LEPTOS_TAILWIND_CONFIG_FILE", tailwind_config);
        }
    }

    let opts = Opts {
        release: config.project.release,
        precompress: config.project.precompress,
        hot_reload: config.project.hot_reload,
        project: None,
        features: config.project.features.clone(),
        lib_features: vec![],
        lib_cargo_args: None,
        bin_features: vec![],
        bin_cargo_args: None,
        wasm_debug: config.project.wasm_debug,
        verbose: config.project.verbose,
        clear: false,
        js_minify: config.project.js_minify,
        split: config.project.split,
        frontend_only: config.project.frontend_only,
        server_only: config.project.server_only,
    };

    let command = match cmd {
        "end-to-end" => Commands::EndToEnd(opts),
        "serve" => Commands::Serve(BinOpts { opts, bin_args: args.to_vec() }),
        "build" => Commands::Build(opts),
        "test" => Commands::Test(opts),
        "watch" => Commands::Watch(BinOpts { opts, bin_args: args.to_vec() }),
        _ => bail!("Unknown cargo-leptos command: {}", cmd),
    };

    let cli = Cli {
        manifest_path: None,
        log: vec![], // TODO: map logs if needed
        command,
    };

    cargo_leptos::run(cli).await
}
