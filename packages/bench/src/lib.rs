//! Professional-grade benchmarking utilities for MontRS.
//!
//! This crate provides tools for measuring performance, gathering system statistics,
//! and generating detailed reports.

pub mod config;
pub mod parameter;
pub mod report;
pub mod runner;
pub mod stats;
pub mod sys;
pub mod weights;

pub use config::BenchConfig;
pub use parameter::{Parameter, ParametricBench};
pub use report::Report;
pub use runner::{BenchRunner, Benchmark};
pub use weights::Weight;

use std::future::Future;

/// Defines a benchmark case.
#[async_trait::async_trait]
pub trait BenchCase: Send + Sync {
    /// The name of the benchmark.
    fn name(&self) -> &str;

    /// Optional parameter info for regression testing.
    fn parameter(&self) -> Option<Parameter> {
        None
    }

    /// Set the current parameter value (if applicable).
    fn set_parameter(&self, _value: u32) {}

    /// Optional setup phase (not timed).
    async fn setup(&self) -> anyhow::Result<()> {
        Ok(())
    }

    /// The workload to measure.
    async fn run(&self) -> anyhow::Result<()>;

    /// Optional teardown phase (not timed).
    async fn teardown(&self) -> anyhow::Result<()> {
        Ok(())
    }
}

/// A wrapper for simple closure-based benchmarks.
pub struct SimpleBench<F, Fut>
where
    F: Fn() -> Fut + Send + Sync,
    Fut: Future<Output = anyhow::Result<()>> + Send,
{
    name: String,
    func: F,
}

impl<F, Fut> SimpleBench<F, Fut>
where
    F: Fn() -> Fut + Send + Sync,
    Fut: Future<Output = anyhow::Result<()>> + Send,
{
    pub fn new(name: impl Into<String>, func: F) -> Self {
        Self {
            name: name.into(),
            func,
        }
    }
}

#[async_trait::async_trait]
impl<F, Fut> BenchCase for SimpleBench<F, Fut>
where
    F: Fn() -> Fut + Send + Sync,
    Fut: Future<Output = anyhow::Result<()>> + Send,
{
    fn name(&self) -> &str {
        &self.name
    }

    async fn run(&self) -> anyhow::Result<()> {
        (self.func)().await
    }
}
