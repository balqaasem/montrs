use crate::config::MontConfig;
use std::sync::Arc;

    let config = MontConfig::load()?;
    let mut leptos_config = config.to_leptos_config(true)?;

    // Pure Rust Tailwind Watcher
    if let Some(input_file) = &config.build.tailwind_input_file {
        if config.build.tailwind_config_file.is_none() {
             println!("Starting Tailwind Watcher (Pure Rust/railwind)...");
             let input_file = input_file.clone();
             let output_path = std::path::PathBuf::from("target/site/pkg/app.css");
             
             // Initial compile
             if let Err(e) = crate::compile::railwind::compile(std::path::Path::new("."), &output_path) {
                 eprintln!("Railwind initial compile failed: {}", e);
             }

             // Spawn watcher thread
             std::thread::spawn(move || {
                 use notify::{Watcher, RecursiveMode, Result};
                 let (tx, rx) = std::sync::mpsc::channel();
                 let mut watcher = notify::recommended_watcher(tx).unwrap();
                 if let Err(e) = watcher.watch(std::path::Path::new("src"), RecursiveMode::Recursive) {
                     eprintln!("Failed to watch src for railwind: {}", e);
                     return;
                 }
                 
                 for res in rx {
                     match res {
                         Ok(_) => {
                             // Simple debounce/recompile
                             if let Err(e) = crate::compile::railwind::compile(std::path::Path::new("."), &output_path) {
                                 eprintln!("Railwind recompile failed: {}", e);
                             } else {
                                 // println!("Railwind recompiled.");
                             }
                         },
                         Err(e) => eprintln!("Watch error: {:?}", e),
                     }
                 }
             });
        }
    }

    let project = leptos_config.current_project()?;
    cargo_leptos::command::watch(&project).await
}
