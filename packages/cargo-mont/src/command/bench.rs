//! Bench command handler.
//!
//! Provides two modes of operation:
//! 1. **Native Mode (`--simple`)**: Benchmarks standalone files, binaries, or AppSpecs directly
//!    without the overhead of creating a temporary cargo project. It reports file size and execution speed.
//! 2. **Standard Mode**: Runs `cargo bench` within an existing project, passing filters and configuration
//!    to the underlying benchmarking harness.

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::fs;
use std::time::Instant;

pub async fn run(
    target: Option<String>,
    iterations: u32,
    warmup: u32,
    timeout: Option<u64>,
    filter: Option<String>,
    json_output: Option<String>,
    simple: bool,
) -> Result<()> {
    if simple {
        if let Some(target_path) = &target {
            // Handle explicit targets without cargo project overhead
            return run_native_bench(target_path, iterations, warmup).await;
        } else {
             anyhow::bail!("--simple mode requires a target file or directory.");
        }
    }

    // Default behavior: run cargo bench
    run_cargo_bench(target, iterations, warmup, timeout, filter, json_output).await
}

async fn run_native_bench(
    target_path: &str,
    iterations: u32,
    warmup: u32,
) -> Result<()> {
    let path = Path::new(target_path);
    if !path.exists() {
        anyhow::bail!("Target path does not exist: {}", target_path);
    }

    // 1. Report File Size
    let metadata = fs::metadata(path)?;
    let size_bytes = metadata.len();
    let size_mb = size_bytes as f64 / 1024.0 / 1024.0;
    
    println!("Target: {}", target_path);
    println!("Size:   {:.2} MB ({} bytes)", size_mb, size_bytes);

    // 2. Identify Target Type
    if path.is_dir() || path.file_name() == Some(std::ffi::OsStr::new("mont.toml")) {
        // AppSpec or Application Directory
        println!("Type:   AppSpec / Application");
        println!("Action: Benchmarking internal config load speed...");
        return bench_appspec_load(path, iterations, warmup).await;
    } 
    
    if path.extension().map_or(false, |e| e == "rs") {
        // Rust Source File
        println!("Type:   Rust Source");
        println!("Action: Compiling and benchmarking execution...");
        return bench_rust_source(path, iterations, warmup).await;
    }

    // Assume Binary / Executable
    println!("Type:   Executable / Binary");
    println!("Action: Benchmarking execution speed...");
    bench_executable(path, iterations, warmup).await
}

/// Benchmarks loading of an AppSpec (mont.toml).
/// 
/// This is an internal benchmark that measures how fast `cargo-mont` can parse the configuration.
async fn bench_appspec_load(path: &Path, iterations: u32, warmup: u32) -> Result<()> {
    // Determine the mont.toml path
    let config_path = if path.is_dir() {
        path.join("mont.toml")
    } else {
        path.to_path_buf()
    };

    if !config_path.exists() {
        anyhow::bail!("mont.toml not found at {}", config_path.display());
    }

    // Warmup
    for _ in 0..warmup {
        let _ = crate::config::MontConfig::from_file(&config_path);
    }

    // Measure
    let mut total_duration = std::time::Duration::new(0, 0);
    for _ in 0..iterations {
        let start = Instant::now();
        let _ = crate::config::MontConfig::from_file(&config_path)?;
        total_duration += start.elapsed();
    }

    let avg_duration = total_duration / iterations;
    println!("Result: {:.4} ms / load (avg over {} runs)", avg_duration.as_secs_f64() * 1000.0, iterations);
    
    Ok(())
}

async fn bench_rust_source(path: &Path, iterations: u32, warmup: u32) -> Result<()> {
    // Attempt to compile using rustc to a temp binary
    // Note: This only works for standalone files without external crate dependencies (except std)
    // If the file relies on `montrs_bench`, this will fail unless we link it manually.
    // Given the constraint "no project creation", we rely on simple `rustc`.
    
    let temp_dir = std::env::temp_dir();
    let file_stem = path.file_stem().unwrap().to_string_lossy();
    let binary_name = if cfg!(windows) { format!("{}.exe", file_stem) } else { file_stem.to_string() };
    let binary_path = temp_dir.join(&binary_name);

    println!("Compiling {}...", path.display());
    let status = Command::new("rustc")
        .arg(path)
        .arg("-o")
        .arg(&binary_path)
        .arg("-O") // Release optimization
        .status()
        .context("Failed to invoke rustc")?;

    if !status.success() {
        println!("Standard compilation failed. If this file uses external crates (like montrs_bench),");
        println!("please run `cargo bench` within a project, or compile it manually first.");
        anyhow::bail!("Compilation failed");
    }

    // Run the produced binary
    bench_executable(&binary_path, iterations, warmup).await
}

async fn bench_executable(path: &Path, iterations: u32, warmup: u32) -> Result<()> {
    // Warmup
    for _ in 0..warmup {
        let _ = Command::new(path).output()?;
    }

    // Measure
    let mut total_duration = std::time::Duration::new(0, 0);
    for _ in 0..iterations {
        let start = Instant::now();
        let _ = Command::new(path).output()?;
        total_duration += start.elapsed();
    }

    let avg_duration = total_duration / iterations;
    println!("Result: {:.4} ms / run (avg over {} runs)", avg_duration.as_secs_f64() * 1000.0, iterations);

    Ok(())
}


async fn run_cargo_bench(
    target: Option<String>,
    iterations: u32,
    warmup: u32,
    timeout: Option<u64>,
    filter: Option<String>,
    json_output: Option<String>,
) -> Result<()> {
    // ... existing logic ...
    let mut cmd = Command::new("cargo");
    cmd.arg("bench");

    // Pass target as positional argument if provided (acts as a filter in cargo bench)
    if let Some(t) = target {
        cmd.arg(t);
    }

    // Arguments passed to the benchmark binary
    let mut harness_args = Vec::new();
    
    if let Some(f) = filter {
        cmd.arg(&f);
    }
    
    cmd.arg("--");

    harness_args.push(format!("--iterations={}", iterations));
    harness_args.push(format!("--warmup={}", warmup));
    
    if let Some(t) = timeout {
        harness_args.push(format!("--timeout={}", t));
    }
    
    if let Some(json) = &json_output {
        harness_args.push(format!("--json-output={}", json));
    }

    cmd.args(&harness_args);

    // Also set Env vars as a fallback/alternative
    cmd.env("MONTRS_BENCH_ITERATIONS", iterations.to_string());
    cmd.env("MONTRS_BENCH_WARMUP", warmup.to_string());
    if let Some(t) = timeout {
        cmd.env("MONTRS_BENCH_TIMEOUT", t.to_string());
    }
    if let Some(json) = &json_output {
        cmd.env("MONTRS_BENCH_JSON_OUTPUT", json);
    }

    // Legacy support (for older binaries using MONT_BENCH_*)
    cmd.env("MONT_BENCH_ITERATIONS", iterations.to_string());
    cmd.env("MONT_BENCH_WARMUP", warmup.to_string());
    if let Some(t) = timeout {
        cmd.env("MONT_BENCH_TIMEOUT", t.to_string());
    }
    if let Some(json) = &json_output {
        cmd.env("MONT_BENCH_JSON_OUTPUT", json);
    }

    let status = cmd.status().context("Failed to execute cargo bench")?;

    if !status.success() {
        anyhow::bail!("Benchmarks failed");
    }

    Ok(())
}
