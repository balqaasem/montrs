use montrs_test::e2e::{MontDriver, assertions};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let driver = MontDriver::new().await?;

    driver.goto("/").await?;
    println!("Successfully navigated to {}", driver.url());

    // Basic assertion example for Todo app
    // assertions::assert_element_exists(&driver.page, "input[type='text']").await?;

    driver.close().await?;

    Ok(())
}
