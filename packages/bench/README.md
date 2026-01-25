# montrs-bench

Professional-grade benchmarking utilities for the MontRS ecosystem.

## Overview

`montrs-bench` provides a robust foundation for measuring the performance of MontRS applications and libraries. It includes:

- **High-Resolution Timing**: Uses `std::time::Instant` for precise measurements.
- **Statistical Analysis**: Calculates Mean, Median, StdDev, P95, P99, and Ops/Sec.
- **System Profiling**: Captures OS, CPU, RAM, and Rust version for context.
- **Reporting**: Supports colored CLI output and detailed JSON export.

## Usage

### Simple Benchmarks

```rust
use montrs_bench::{BenchRunner, SimpleBench};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut runner = BenchRunner::new();

    runner.add(SimpleBench::new("vector_sort", || async {
        let mut v = vec![5, 2, 8, 1, 9];
        v.sort();
        Ok(())
    }));

    runner.run().await
}
```

### Advanced Benchmarks

Implement the `BenchCase` trait for setup/teardown logic:

```rust
use montrs_bench::BenchCase;
use async_trait::async_trait;

struct MyBench;

#[async_trait]
impl BenchCase for MyBench {
    fn name(&self) -> &str { "complex_bench" }
    
    async fn setup(&self) -> anyhow::Result<()> {
        // Prepare resources
        Ok(())
    }

    async fn run(&self) -> anyhow::Result<()> {
        // Measure this
        Ok(())
    }
}
```

## Integration with MontRS

This crate is used by `cargo mont bench` to execute performance tests across the framework.
