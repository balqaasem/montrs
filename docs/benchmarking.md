# Benchmarking in MontRS

MontRS provides a professional-grade benchmarking ecosystem powered by the `montrs-bench` package. This allows you to measure the performance of your application and individual components with statistical rigor.

## Overview

The benchmarking system is designed to be:
- **Accurate**: Uses high-resolution timers and statistical analysis (mean, median, p95/p99, std dev).
- **System-Aware**: Captures detailed system information (OS, CPU, RAM) for context.
- **Integrated**: Works seamlessly with `cargo mont bench` and standard Rust tests.
- **Exportable**: Supports JSON reporting for CI/CD integration.

## Quick Start

### 1. Add Dependency

Add `montrs-bench` to your `Cargo.toml` (usually in `[dev-dependencies]`):

```toml
[dev-dependencies]
montrs-bench = "0.1.0"
```

### 2. Write a Benchmark

You can define benchmarks using the `SimpleBench` wrapper or by implementing the `BenchCase` trait.

```rust
use montrs_bench::{BenchRunner, SimpleBench, BenchConfig};

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

### 3. Run via CLI

Use the `cargo mont bench` command to execute benchmarks.

```bash
# Run all benchmarks
cargo mont bench

# Customize execution
cargo mont bench --iterations 1000 --warmup 50 --timeout 60

# Filter specific benchmarks
cargo mont bench --filter "vector_sort"

# Export report
cargo mont bench --json-output report.json
```

---

## Native Benchmarking (`--simple`)

MontRS supports a high-performance "native" mode for benchmarking standalone files, binaries, or `mont.toml` configurations without the overhead of a full Cargo project.

This is ideal for quick testing, measuring script execution, or checking the size and load time of an AppSpec.

### Features
- **Zero Configuration**: No `Cargo.toml` required.
- **File Size Reporting**: Automatically reports the target's size in MB and bytes.
- **Smart Compilation**: Standalone `.rs` files are compiled with `rustc -O` and benchmarked.
- **Direct Binary Execution**: Benchmarks existing executables directly.
- **Config Profiling**: Measures `mont.toml` (AppSpec) parsing and load speed.

### Usage Examples

```bash
# Simple bench of a Rust source file
cargo mont bench --simple src/utils.rs

# Bench an existing binary
cargo mont bench --simple ./target/release/my-app

# Profile AppSpec load time
cargo mont bench --simple ./mont.toml

# Customize iterations and warmup for native mode
cargo mont bench --simple ./script.rs --iterations 50 --warmup 5
```

> **Note**: For `.rs` files, native mode uses `rustc` directly. If your file depends on external crates (other than `std`), you should use the standard project-based `cargo mont bench` instead.

---

## Writing Benchmarks

### Closure-Based Benchmarks

For simple cases, use `SimpleBench`:

```rust
use montrs_bench::SimpleBench;

let bench = SimpleBench::new("db_query", || async {
    db.query("SELECT 1").await?;
    Ok(())
});
```

### Advanced Benchmarks (Trait-Based)

For more control over setup and teardown, implement `BenchCase`:

```rust
use montrs_bench::BenchCase;
use async_trait::async_trait;

struct DbBenchmark {
    db: Database,
}

#[async_trait]
impl BenchCase for DbBenchmark {
    fn name(&self) -> &str {
        "complex_db_operation"
    }

    async fn setup(&self) -> anyhow::Result<()> {
        self.db.connect().await?;
        self.db.seed_data().await?;
        Ok(())
    }

    async fn run(&self) -> anyhow::Result<()> {
        self.db.perform_heavy_calc().await?;
        Ok(())
    }

    async fn teardown(&self) -> anyhow::Result<()> {
        self.db.cleanup().await?;
        Ok(())
    }
}
```

## Parameter-based Benchmarking (Substrate-Inspired)

MontRS supports advanced parameter-based benchmarking, inspired by the **Substrate** blockchain framework. This allows you to measure how performance scales as inputs change (e.g., data size, number of elements, or iteration counts).

### Parametric Benchmarking

A parametric benchmark runs a workload across a range of values for a specific parameter. The runner then performs **Linear Regression** to calculate the **Slope** (time per unit of the parameter) and the **Intercept** (fixed overhead).

#### 1. Defining a Parametric Benchmark

Use `ParametricBench` to wrap a function that takes a `u32` parameter:

```rust
use montrs_bench::{BenchRunner, ParametricBench, Parameter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut runner = BenchRunner::new();

    // Define a parameter 'n' from 100 to 1000 with step 100
    let param = Parameter::new("n", 100..=1000, 100);

    runner.add(ParametricBench::new("vector_fill", param, |n| async move {
        let mut v = Vec::with_capacity(n as usize);
        for i in 0..n {
            v.push(i);
        }
        Ok(())
    }));

    runner.run().await
}
```

#### 2. Analyzing Results

The output will include the linear regression results:

```text
Running vector_fill...
  Parameter: n (10 values, 10 runs/val)
  Mean:    54.2300 µs
  Slope:   0.0542 ns/param  <-- Time scaling per unit of 'n'
  Intercept: 1.2000 µs      <-- Base overhead
```

### Hardware Awareness

Benchmarks in MontRS capture detailed hardware information to ensure results are reproducible and comparable across different machines. The report includes:
- **CPU Frequency**: Useful for identifying thermal throttling.
- **Physical vs Logical Cores**: Helps understand multi-threading performance.
- **Memory Bandwidth context**: (Captured via system RAM specs).
- **Binary Size**: Tracks the footprint of the benchmarked code.

### Weight Generation

Similar to Substrate's "weights", you can use these metrics to assign "costs" to operations in your application based on their performance scaling. This is critical for building resource-constrained systems or billing for computation.

---

## Configuration

You can configure the benchmark runner programmatically or via CLI args.

```rust
use montrs_bench::BenchConfig;
use std::time::Duration;

let config = BenchConfig {
    iterations: 500,
    warmup_iterations: 20,
    duration: Some(Duration::from_secs(10)),
    ..Default::default()
};

let runner = BenchRunner::with_config(config);
```

## Reports

The CLI output provides a human-readable summary:

```text
Running MontRS Benchmarks
System: Windows (Intel(R) Core(TM) i9-14900K)
---------------------------------------------------
Running vector_sort... Done
  Mean:    1.2345 µs
  Median:  1.2000 µs
  StdDev:  0.0500 µs
  Ops/sec: 810044.55
---------------------------------------------------
```

The JSON report (`--json-output`) contains detailed data for analysis:

```json
{
  "system": {
    "os_name": "Windows",
    "cpu_cores": 24,
    "rust_version": "1.75.0"
  },
  "results": {
    "vector_sort": {
      "stats": {
        "mean": 0.0000012345,
        "p99": 0.0000015000
      },
      "iterations": 100
    }
  }
}
```

## Integration with `montrs-test`

The `montrs-test` package re-exports benchmarking utilities for convenience in unit tests.

```rust
use montrs_test::unit::bench;

#[tokio::test]
async fn test_perf() {
    bench("quick_check", 100, || async {
        // ...
    }).await;
}
```
