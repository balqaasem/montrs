use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Configuration for benchmark execution.
///
/// Can be loaded from environment variables `MONT_BENCH_*` or created programmatically.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchConfig {
    /// Number of warm-up iterations.
    /// Env: `MONT_BENCH_WARMUP`
    pub warmup_iterations: u32,
    /// Number of measurement iterations.
    /// Env: `MONT_BENCH_ITERATIONS`
    pub iterations: u32,
    /// Maximum duration for the benchmark (optional).
    /// Env: `MONT_BENCH_TIMEOUT`
    pub duration: Option<Duration>,
    /// Filter string to run specific benchmarks.
    pub filter: Option<String>,
    /// Path to export JSON report.
    /// Env: `MONT_BENCH_JSON_OUTPUT`
    pub json_output: Option<String>,
}

impl Default for BenchConfig {
    fn default() -> Self {
        // Try to load from env, otherwise use defaults
        let warmup_iterations = std::env::var("MONT_BENCH_WARMUP")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(10);
            
        let iterations = std::env::var("MONT_BENCH_ITERATIONS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(100);
            
        let duration = std::env::var("MONT_BENCH_TIMEOUT")
            .ok()
            .and_then(|v| v.parse().ok())
            .map(Duration::from_secs);
            
        let json_output = std::env::var("MONT_BENCH_JSON_OUTPUT").ok();

        Self {
            warmup_iterations,
            iterations,
            duration: duration.or(Some(Duration::from_secs(5))),
            filter: None,
            json_output,
        }
    }
}
