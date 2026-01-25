//! Bench command handler.

use anyhow::{Context, Result};
use std::process::Command;

pub async fn run(
    iterations: u32,
    warmup: u32,
    timeout: Option<u64>,
    filter: Option<String>,
    json_output: Option<String>,
) -> Result<()> {
    // We delegate to `cargo bench` but we need to pass arguments to the test binary.
    // Standard `cargo bench` compiles and runs benchmarks.
    // To pass args to the harness, we put them after `--`.
    
    // However, since we are building a custom benchmark harness (montrs-bench),
    // we need to ensure the user's benchmarks are using it.
    
    // For now, let's assume the user has configured their project to use
    // the montrs-bench harness or standard libtest harness.
    // If they use montrs-bench, they should be able to parse these args.
    
    // But `cargo bench` args are specific to libtest.
    // If montrs-bench acts as a library, users likely write:
    // #[bench] or use a custom main.
    
    // Strategy: Run `cargo bench` and pass our args.
    // If the underlying harness is libtest, some args might not match exactly,
    // but `iterations` is usually controlled via CLI in libtest? No, libtest is mostly auto-tuning.
    
    // If we want "Professional-grade" control, we might need to recommend
    // using `cargo run --release --bin bench_runner` pattern, OR
    // we rely on `cargo bench` and pass args that our `montrs-bench` library parses
    // if it's used as the harness.
    
    // Let's construct the command.
    let mut cmd = Command::new("cargo");
    cmd.arg("bench");

    // Arguments passed to the benchmark binary
    let mut harness_args = Vec::new();

    // These args need to be supported by the target benchmark harness.
    // Since we control `montrs-bench`, we can define them there.
    // But standard `cargo bench` (libtest) doesn't support all of them easily.
    
    // For `cargo-mont bench` to work seamlessly, we will pass them as env vars
    // or CLI args. CLI args are safer.
    
    // Note: `cargo bench` separates cargo args from binary args with `--`.
    
    if let Some(f) = filter {
        // Cargo bench takes filter as a positional arg
        cmd.arg(&f);
    }
    
    cmd.arg("--");

    // Pass custom args for montrs-bench
    // We pass arguments that the `BenchRunner` in `montrs-bench` should parse.
    // Since `montrs-bench` is a library, the user's benchmark binary (or test harness)
    // is responsible for parsing these if they use `BenchRunner::from_args()` or similar.
    // However, currently `BenchRunner` relies on `BenchConfig`.
    // We should probably implement `clap` parsing in `montrs-bench` or read env vars.
    // For now, we rely on environment variables as the primary configuration method
    // because it's non-intrusive to the binary's argument parsing logic if it uses libtest.
    
    // We still pass them as args for visibility in `ps`, but env vars are the contract.
    harness_args.push(format!("--iterations={}", iterations));
    harness_args.push(format!("--warmup={}", warmup));
    
    if let Some(t) = timeout {
        harness_args.push(format!("--timeout={}", t));
    }
    
    if let Some(json) = json_output {
        harness_args.push(format!("--json-output={}", json));
    }

    cmd.args(&harness_args);

    // Also set Env vars as a fallback/alternative
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
