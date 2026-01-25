use serde::{Deserialize, Serialize};
use statrs::statistics::{Data, Distribution, Max, Min, OrderStatistics, Statistics};
use std::time::Duration;

/// Statistical analysis of benchmark results.
///
/// This struct holds key performance metrics calculated from a series of measurements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchStats {
    /// Arithmetic mean of execution times (seconds).
    pub mean: f64,
    /// Median execution time (seconds).
    pub median: f64,
    /// Minimum execution time observed (seconds).
    pub min: f64,
    /// Maximum execution time observed (seconds).
    pub max: f64,
    /// Standard deviation of execution times.
    pub std_dev: f64,
    /// 95th percentile execution time.
    pub p95: f64,
    /// 99th percentile execution time.
    pub p99: f64,
    /// Estimated operations per second (throughput).
    pub ops_per_sec: f64,
}

impl BenchStats {
    /// Calculates statistics from a slice of durations.
    ///
    /// # Arguments
    ///
    /// * `durations` - The raw timing data from benchmark iterations.
    pub fn new(durations: &[Duration]) -> Self {
        let mut data: Vec<f64> = durations.iter().map(|d| d.as_secs_f64()).collect();
        let mut stats_data = Data::new(&mut data);

        let mean = stats_data.mean().unwrap_or(0.0);
        let std_dev = stats_data.std_dev().unwrap_or(0.0);
        let median = stats_data.median();
        let min = stats_data.min();
        let max = stats_data.max();
        let p95 = stats_data.percentile(95);
        let p99 = stats_data.percentile(99);

        let ops_per_sec = if mean > 0.0 { 1.0 / mean } else { 0.0 };

        Self {
            mean,
            median,
            min,
            max,
            std_dev,
            p95,
            p99,
            ops_per_sec,
        }
    }
}
