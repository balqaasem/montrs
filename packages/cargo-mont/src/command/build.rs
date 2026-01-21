use crate::config::MontConfig;

pub async fn run() -> anyhow::Result<()> {
    let config = MontConfig::load()?;
    let mut leptos_config = config.to_leptos_config(false)?;

    // Pure Rust Tailwind Compilation (Railwind)
    // If we have a tailwind-input-file but NO tailwind-config-file (or explicit instruction), use railwind
    // Actually, `cargo-leptos` requires tailwind-config-file to run standard tailwind.
    // So if user wants pure rust, they can omit tailwind-config-file and we handle it here.
    
    if let Some(input_file) = &config.build.tailwind_input_file {
         if config.build.tailwind_config_file.is_none() {
            println!("Compiling Tailwind (Pure Rust/railwind)...");
            let input_path = std::path::Path::new(input_file);
            // Default output path consistent with cargo-leptos behavior or site-root
            let output_path = std::path::Path::new("target/site/pkg/app.css"); 
            
            crate::compile::railwind::compile(std::path::Path::new("."), output_path)?;
            println!("Tailwind compiled to {:?}", output_path);

            // Inject the generated CSS into leptos config so it includes it in the final artifact
             // We modify the Project in the leptos config
             if let Some(project) = leptos_config.projects.get_mut(0) {
                 // We need to cast Arc to make it mutable or replace it.
                 // cargo-leptos::Config uses Arc<Project>. We can clone/modify if we had mutable access.
                 // Actually, we can just rebuild the project struct or use the specific fields if accessible.
                 
                 // Since we can't easily mutate the Arc inside Config, we might need to rely on the side-effect 
                 // that we wrote the file to `target/site/pkg/app.css`.
                 // But wait, cargo-leptos needs to know about this file to serve/hash it?
                 // Or we can set `style_file` in mont.toml to point to this generated file?
                 
                 // Better approach: User configures `style-file = "target/site/pkg/app.css"` in mont.toml?
                 // OR we programmatically set it here if we can.
                 // Since to_leptos_config created the Project, we can modify it *before* returning if we change the signature 
                 // or just accept that to_leptos_config might need to be smart.
             }
         }
    }

    cargo_leptos::command::build_all(&leptos_config).await
}
