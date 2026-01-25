use crate::stats::BenchStats;
use crate::sys::SystemInfo;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A full benchmark report containing system info and results.
///
/// This structure is serializable to JSON for external analysis.
#[derive(Debug, Serialize, Deserialize)]
pub struct Report {
    /// Information about the system where the benchmark was run.
    pub system: SystemInfo,
    /// A map of benchmark names to their results.
    pub results: HashMap<String, BenchResult>,
    /// The timestamp when the report was created.
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// The result of a single benchmark execution.
#[derive(Debug, Serialize, Deserialize)]
pub struct BenchResult {
    /// Statistical analysis of the run.
    pub stats: BenchStats,
    /// Total number of iterations performed.
    pub iterations: u32,
    /// Total wall-clock time for all iterations (excluding warmup).
    pub total_duration_secs: f64,
}

impl Report {
    /// Creates a new, empty report with current system info.
    pub fn new() -> Self {
        Self {
            system: SystemInfo::collect(),
            results: HashMap::new(),
            timestamp: chrono::Utc::now(),
        }
    }

    /// Adds a result to the report.
    pub fn add_result(&mut self, name: String, stats: BenchStats, iterations: u32, total_duration_secs: f64) {
        self.results.insert(name, BenchResult {
            stats,
            iterations,
            total_duration_secs,
        });
    }

    /// Saves the report to a JSON file.
    pub fn save_json(&self, path: &str) -> anyhow::Result<()> {
        let file = std::fs::File::create(path)?;
        serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }
}
