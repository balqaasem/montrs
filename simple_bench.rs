use montrs_bench::{BenchRunner, SimpleBench};
use std::time::Duration;

pub async fn run() -> anyhow::Result<()> {
    tokio::time::sleep(Duration::from_millis(50)).await;
    Ok(())
}
