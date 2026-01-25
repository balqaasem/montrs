use crate::config::BenchConfig;
use crate::report::Report;
use crate::stats::BenchStats;
use crate::BenchCase;
use colored::*;
use std::time::{Duration, Instant};

/// The main entry point for running benchmarks.
///
/// `BenchRunner` orchestrates the execution of registered benchmarks.
/// It handles configuration, setup/teardown, warmup, measurement, and reporting.
///
/// # Example
///
/// ```rust
/// use montrs_bench::{BenchRunner, Benchmark};
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let mut runner = BenchRunner::new();
///     runner.add(Benchmark::new("my_bench", || async {
///         // ... workload ...
///         Ok(())
///     }));
///     runner.run().await
/// }
/// ```
pub struct BenchRunner {
    config: BenchConfig,
    benchmarks: Vec<Box<dyn BenchCase>>,
}

impl BenchRunner {
    /// Creates a new `BenchRunner` with default configuration.
    pub fn new() -> Self {
        Self {
            config: BenchConfig::default(),
            benchmarks: Vec::new(),
        }
    }

    /// Creates a new `BenchRunner` with custom configuration.
    pub fn with_config(config: BenchConfig) -> Self {
        Self {
            config,
            benchmarks: Vec::new(),
        }
    }

    /// Adds a benchmark to the runner.
    pub fn add<B: BenchCase + 'static>(&mut self, benchmark: B) -> &mut Self {
        self.benchmarks.push(Box::new(benchmark));
        self
    }

    /// Executes all registered benchmarks and reports results.
    ///
    /// This method will:
    /// 1. Print system information.
    /// 2. Iterate through benchmarks (filtering if configured).
    /// 3. Run setup, warmup, measurement loop, and teardown for each.
    /// 4. Print results to stdout.
    /// 5. Optionally save a JSON report.
    pub async fn run(&self) -> anyhow::Result<()> {
        let mut report = Report::new();

        println!("{}", "Running MontRS Benchmarks".bold().green());
        println!("System: {} ({})", report.system.os_name, report.system.cpu_brand);
        println!("---------------------------------------------------");

        for bench in &self.benchmarks {
            if let Some(filter) = &self.config.filter {
                if !bench.name().contains(filter) {
                    continue;
                }
            }

            self.run_single_bench(bench.as_ref(), &mut report).await?;
        }

        if let Some(path) = &self.config.json_output {
            report.save_json(path)?;
            println!("Report saved to {}", path.blue());
        }

        Ok(())
    }

    /// Internal method to run a single benchmark case.
    async fn run_single_bench(&self, bench: &dyn BenchCase, report: &mut Report) -> anyhow::Result<()> {
        print!("Running {}... ", bench.name().cyan());
        
        bench.setup().await?;

        // Warmup phase: Run without measuring to prime caches/JIT
        for _ in 0..self.config.warmup_iterations {
            bench.run().await?;
        }

        let mut durations = Vec::with_capacity(self.config.iterations as usize);
        let start_total = Instant::now();

        // Measurement phase
        for _ in 0..self.config.iterations {
            let start = Instant::now();
            bench.run().await?;
            durations.push(start.elapsed());

            // Check for timeout
            if let Some(max_dur) = self.config.duration {
                if start_total.elapsed() > max_dur {
                    break;
                }
            }
        }

        let total_duration = start_total.elapsed();
        bench.teardown().await?;

        let stats = BenchStats::new(&durations);
        report.add_result(bench.name().to_string(), stats.clone(), durations.len() as u32, total_duration.as_secs_f64());

        println!("{}", "Done".green());
        println!("  Mean:    {:.4} µs", stats.mean * 1_000_000.0);
        println!("  Median:  {:.4} µs", stats.median * 1_000_000.0);
        println!("  StdDev:  {:.4} µs", stats.std_dev * 1_000_000.0);
        println!("  Ops/sec: {:.2}", stats.ops_per_sec);
        println!("---------------------------------------------------");

        Ok(())
    }
}

/// A convenience wrapper to define benchmarks using the `BenchCase` trait.
pub struct Benchmark<F, Fut> 
where
    F: Fn() -> Fut + Send + Sync,
    Fut: std::future::Future<Output = anyhow::Result<()>> + Send,
{
    name: String,
    func: F,
}

impl<F, Fut> Benchmark<F, Fut> 
where
    F: Fn() -> Fut + Send + Sync,
    Fut: std::future::Future<Output = anyhow::Result<()>> + Send,
{
    pub fn new(name: &str, func: F) -> Self {
        Self {
            name: name.to_string(),
            func,
        }
    }
}

#[async_trait::async_trait]
impl<F, Fut> BenchCase for Benchmark<F, Fut> 
where
    F: Fn() -> Fut + Send + Sync,
    Fut: std::future::Future<Output = anyhow::Result<()>> + Send,
{
    fn name(&self) -> &str {
        &self.name
    }

    async fn run(&self) -> anyhow::Result<()> {
        (self.func)().await
    }
}
