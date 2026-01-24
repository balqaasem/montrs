//! Unit testing primitives for MontRS.
//!
//! This module contains traits and helpers for writing unit tests that require
//! setup and teardown logic, as well as simple benchmarking utilities.

use async_trait::async_trait;
use std::time::Instant;

/// A trait for defining test fixtures with setup and teardown logic.
///
/// Implement this trait to manage resources that are needed for a test case,
/// such as database connections, temporary files, or mock servers.
///
/// # Example
///
/// ```rust
/// use montrs_test::unit::Fixture;
/// use async_trait::async_trait;
///
/// struct TempFileFixture;
///
/// #[async_trait]
/// impl Fixture for TempFileFixture {
///     type Context = std::path::PathBuf;
///
///     async fn setup(&self) -> anyhow::Result<Self::Context> {
///         Ok(std::env::temp_dir().join("test_file"))
///     }
///
///     async fn teardown(&self, path: &mut Self::Context) -> anyhow::Result<()> {
///         if path.exists() {
///             std::fs::remove_file(path)?;
///         }
///         Ok(())
///     }
/// }
/// ```
#[async_trait]
pub trait Fixture {
    /// The type of data produced by setup and used by the test.
    type Context;

    /// Sets up the fixture environment.
    ///
    /// This method is called before the test execution. It should return
    /// the context data needed for the test.
    async fn setup(&self) -> anyhow::Result<Self::Context>;

    /// Cleans up the fixture environment.
    ///
    /// This method is called after the test execution, regardless of whether
    /// the test passed or failed.
    async fn teardown(&self, _context: &mut Self::Context) -> anyhow::Result<()> {
        Ok(())
    }
}

/// Runs a test with the given fixture.
///
/// This helper ensures that `setup` is called before the test,
/// and `teardown` is called after, even if the test fails.
///
/// # Arguments
///
/// * `fixture` - The fixture implementation to use.
/// * `test` - The test closure, which receives a mutable reference to the fixture context.
///
/// # Example
///
/// ```rust
/// use montrs_test::unit::{Fixture, run_fixture_test};
/// use async_trait::async_trait;
///
/// struct MyFixture;
///
/// #[async_trait]
/// impl Fixture for MyFixture {
///     type Context = String;
///     async fn setup(&self) -> anyhow::Result<Self::Context> {
///         Ok("hello".to_string())
///     }
/// }
///
/// #[tokio::test]
/// async fn my_test() -> anyhow::Result<()> {
///     run_fixture_test(MyFixture, |ctx| async move {
///         assert_eq!(ctx, "hello");
///         Ok(())
///     }).await
/// }
/// ```
pub async fn run_fixture_test<F, T, Fut>(fixture: F, test: T) -> anyhow::Result<()>
where
    F: Fixture + Send + Sync,
    F::Context: Send,
    T: FnOnce(&mut F::Context) -> Fut + Send,
    Fut: std::future::Future<Output = anyhow::Result<()>> + Send,
{
    let mut context = fixture.setup().await?;
    
    let result = test(&mut context).await;
    
    // Always run teardown
    let teardown_result = fixture.teardown(&mut context).await;

    // Return the test result first, or the teardown error if test succeeded but teardown failed
    result.and(teardown_result)
}

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
