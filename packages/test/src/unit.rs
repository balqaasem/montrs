//! Unit testing primitives for MontRS.
//!
//! This module contains simple benchmarking utilities and basic assertions.

use std::time::Instant;

/// Simple benchmarking utility.
///
/// Runs the provided async function multiple times and prints timing statistics.
///
/// # Arguments
///
/// * `name` - A label for the benchmark.
/// * `iterations` - The number of times to run the function.
/// * `func` - The async function to benchmark.
pub async fn bench<F, Fut>(name: &str, iterations: u32, func: F) 
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = ()>,
{
    println!("Benchmarking: {}", name);
    let start = Instant::now();
    for _ in 0..iterations {
        func().await;
    }
    let duration = start.elapsed();
    let avg = duration.as_secs_f64() / iterations as f64;
    println!("  Total time: {:.4}s", duration.as_secs_f64());
    println!("  Avg time:   {:.6}s/iter", avg);
}

/// A simple assertion helper for checking if a value matches a predicate.
///
/// # Arguments
///
/// * `value` - The value to check.
/// * `predicate` - A function that returns true if the value matches.
/// * `message` - The error message if the assertion fails.
pub fn assert_that<T, P>(value: &T, predicate: P, message: &str) -> anyhow::Result<()>
where
    P: Fn(&T) -> bool,
{
    if predicate(value) {
        Ok(())
    } else {
        anyhow::bail!("Assertion failed: {}", message)
    }
}
