use montrs_test::e2e::{MontDriver, assertions};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize MontDriver with default config (reads from env vars)
    let driver = MontDriver::new().await?;

    // Navigate to the app (automatically handles base URL)
    driver.goto("/").await?;
    
    println!("Successfully navigated to {}", driver.url());
    
    // Example assertion
    // assertions::assert_title_contains(&driver.page, "MontRS").await?;
    
    // Screenshot for verification
    // driver.screenshot("screenshot.png").await?;
    
    // Cleanup
    driver.close().await?;

    Ok(())
}
