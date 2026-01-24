//! End-to-End (E2E) testing module for MontRS.
//!
//! This module provides a wrapper around `playwright-rs` to facilitate writing
//! E2E tests for MontRS applications. It includes configuration management,
//! test driver orchestration, and helper assertions.
//!
//! # Usage
//!
//! Add `montrs-test` with the `e2e` feature to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! montrs-test = { version = "...", features = ["e2e"] }
//! ```
//!
//! Then use `MontDriver` in your test:
//!
//! ```rust
//! use montrs_test::e2e::MontDriver;
//!
//! #[tokio::test]
//! async fn my_test() -> anyhow::Result<()> {
//!     let driver = MontDriver::new().await?;
//!     driver.goto("/").await?;
//!     // ...
//!     Ok(())
//! }
//! ```

use std::env;
use std::time::Duration;
use playwright::Playwright;
use playwright::api::{Browser, BrowserContext, Page, BrowserType};

// Re-export playwright so users don't need to add it separately if they don't want to
pub use playwright;

/// Configuration for the E2E test session.
#[derive(Debug, Clone)]
pub struct E2EConfig {
    pub headless: bool,
    pub base_url: String,
    pub timeout: u32,
}

impl Default for E2EConfig {
    fn default() -> Self {
        let base_url = env::var("MONT_SITE_URL")
            .or_else(|_| env::var("LEPTOS_SITE_ADDR").map(|addr| format!("http://{}", addr)))
            .unwrap_or_else(|_| "http://127.0.0.1:8080".to_string());

        Self {
            headless: env::var("MONT_E2E_HEADLESS").map(|v| v != "false").unwrap_or(true),
            base_url,
            timeout: 30000,
        }
    }
}

/// A trait for extending MontDriver functionality.
#[async_trait::async_trait]
pub trait MontPlugin {
    /// Called immediately after the driver is initialized.
    async fn on_init(&self, driver: &MontDriver) -> anyhow::Result<()>;
}

/// The main driver for MontRS E2E tests.
/// Wraps Playwright to provide an ergonomic, framework-aware testing experience.
pub struct MontDriver {
    pub playwright: Playwright,
    pub browser: Browser,
    pub context: BrowserContext,
    pub page: Page,
    pub config: E2EConfig,
}

impl MontDriver {
    /// Launches a new browser instance and prepares a page for testing.
    pub async fn new() -> anyhow::Result<Self> {
        Self::with_config(E2EConfig::default()).await
    }

    /// Launches with custom configuration.
    pub async fn with_config(config: E2EConfig) -> anyhow::Result<Self> {
        let playwright = Playwright::initialize().await?;
        playwright.prepare()?; // Install browsers if needed

        // Currently defaulting to Chromium, but could be configurable
        let chromium = playwright.chromium();
        let launcher = chromium.launcher();
        
        let browser = launcher
            .headless(config.headless)
            .timeout(config.timeout as f64)
            .launch()
            .await?;
            
        let context = browser.context_builder().build().await?;
        let page = context.new_page().await?;

        Ok(Self {
            playwright,
            browser,
            context,
            page,
            config,
        })
    }

    /// Register and run a plugin.
    pub async fn use_plugin<P: MontPlugin>(&self, plugin: P) -> anyhow::Result<&Self> {
        plugin.on_init(self).await?;
        Ok(self)
    }

    /// Navigates to a path relative to the configured base URL.
    /// If an absolute URL is provided, it is used as-is.
    pub async fn goto(&self, path: &str) -> anyhow::Result<()> {
        let url = if path.starts_with("http://") || path.starts_with("https://") {
            path.to_string()
        } else {
            // Ensure proper slash handling
            let base = self.config.base_url.trim_end_matches('/');
            let path = path.trim_start_matches('/');
            format!("{}/{}", base, path)
        };
        
        self.page.goto_builder(&url).goto().await?;
        Ok(())
    }

    /// Returns the current URL of the page.
    pub fn url(&self) -> String {
        self.page.url().unwrap_or_default()
    }

    /// Reloads the current page.
    pub async fn reload(&self) -> anyhow::Result<()> {
        self.page.reload_builder().reload().await?;
        Ok(())
    }

    /// Takes a screenshot and saves it to the specified path.
    pub async fn screenshot(&self, path: &str) -> anyhow::Result<()> {
        self.page.screenshot_builder()
            .path(std::path::Path::new(path))
            .screenshot()
            .await?;
        Ok(())
    }

    /// Closes the browser and context.
    pub async fn close(&self) -> anyhow::Result<()> {
        self.context.close().await?;
        self.browser.close().await?;
        Ok(())
    }
}

/// Helper assertions for MontRS applications.
pub mod assertions {
    use playwright::api::Page;

    /// Asserts that the page title contains the expected text.
    pub async fn assert_title_contains(page: &Page, text: &str) -> anyhow::Result<()> {
        let title = page.title().await?;
        if !title.contains(text) {
            anyhow::bail!("Expected title to contain '{}', but found '{}'", text, title);
        }
        Ok(())
    }

    /// Asserts that an element exists on the page.
    pub async fn assert_element_exists(page: &Page, selector: &str) -> anyhow::Result<()> {
        match page.query_selector(selector).await? {
            Some(_) => Ok(()),
            None => anyhow::bail!("Element '{}' not found", selector),
        }
    }
}
