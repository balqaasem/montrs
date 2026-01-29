# Agent Guide: montrs-bench

This guide helps agents measure and optimize performance in MontRS.

## Core Concepts

### 1. BenchCase Trait
Implement this trait to define a benchmark.
- **Setup/Teardown**: Use these for non-timed resource management.
- **Run**: The actual code to be measured.

### 2. BenchRunner
The orchestrator that runs benchmarks and generates reports.

### 3. Statistics
Provides Mean, Median, P95, P99, etc. Use these to identify performance regressions.

## Agent Usage Patterns

### Defining a Benchmark
```rust
pub struct MyBench;

#[async_trait]
impl BenchCase for MyBench {
    fn name(&self) -> &str { "my_feature_performance" }
    async fn run(&self) -> anyhow::Result<()> {
        // Code to measure
        Ok(())
    }
}
```

### Analyzing Results
When a benchmark is run, it generates a `Report`. Use this report to suggest optimizations.
- **High Variance**: Suggests unstable environment or background tasks.
- **High P99**: Suggests occasional blocking operations or resource contention.
