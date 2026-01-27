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
    // Automatically parse args and env vars
    let mut runner = BenchRunner::from_args();

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

## Configuration

`montrs-bench` supports configuration via command-line arguments and environment variables.

### Priority Order
1. Command-line arguments (e.g., `--warmup 20`)
2. Environment variables (e.g., `MONTRS_BENCH_WARMUP=20`)
3. Default values

### Options

| Argument | Env Var | Default | Description |
|----------|---------|---------|-------------|
| `--warmup <N>` | `MONTRS_BENCH_WARMUP` | 10 | Number of warm-up iterations |
| `--iterations <N>` | `MONTRS_BENCH_ITERATIONS` | 100 | Number of measurement iterations |
| `--timeout <S>` | `MONTRS_BENCH_TIMEOUT` | 5 | Max duration in seconds |
| `--filter <STR>` | `MONTRS_BENCH_FILTER` | None | Run only matching benchmarks |
| `--json-output <PATH>` | `MONTRS_BENCH_JSON_OUTPUT` | None | Save results to JSON file |

### Example

```bash
# Run with custom warmup and iterations
cargo run --release --bin my_bench -- --warmup 20 --iterations 500
```

## Integration with MontRS

This crate is used by `montrs bench` to execute performance tests across the framework.
