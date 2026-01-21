use playwright::Playwright;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let playwright = Playwright::initialize().await?; // Initialize Playwright
    playwright.prepare()?; // Install browsers
    
    let chromium = playwright.chromium();
    let browser = chromium.launcher().headless(true).launch().await?;
    let context = browser.context_builder().build().await?;
    let page = context.new_page().await?;

    // Basic test: Navigate to the app and check title/content
    page.goto_builder("http://localhost:3000").goto().await?;
    
    // Example assertion logic (you might want a proper test runner like nextest eventually)
    // For now, we'll just print success
    println!("Successfully navigated to http://localhost:3000");
    
    // Screenshot for verification
    // page.screenshot_builder().path("screenshot.png").screenshot().await?;

    Ok(())
}
