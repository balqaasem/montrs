use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Configuration for benchmark execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchConfig {
    /// Number of warm-up iterations.
    pub warmup_iterations: u32,
    /// Number of measurement iterations.
    pub iterations: u32,
    /// Maximum duration for the benchmark (optional).
    pub duration: Option<Duration>,
    /// Filter string to run specific benchmarks.
    pub filter: Option<String>,
    /// Path to export JSON report.
    pub json_output: Option<String>,
}

impl Default for BenchConfig {
    fn default() -> Self {
        Self {
            warmup_iterations: 10,
            iterations: 100,
            duration: Some(Duration::from_secs(5)),
            filter: None,
            json_output: None,
        }
    }
}
