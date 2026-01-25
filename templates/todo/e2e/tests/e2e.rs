use montrs_test::e2e::{MontDriver, assertions};

#[tokio::test]
async fn test_app_is_running() -> anyhow::Result<()> {
    // Initialize MontDriver with default config (reads from env vars)
    let driver = MontDriver::new().await?;

    // Navigate to the app (automatically handles base URL)
    driver.goto("/").await?;
    
    // Check if we are on the right page
    println!("Successfully navigated to {}", driver.url());
    
    // Example assertion: Uncomment when you have a specific title to check
    // assertions::assert_title_contains(&driver.page, "MontRS").await?;
    
    // Cleanup
    driver.close().await?;

    Ok(())
}
