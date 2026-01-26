//! Configuration module for MontRS.
//!
//! This module defines the structure of the `mont.toml` configuration file
//! and handles loading/parsing logic. It serves as the central source of truth
//! for project settings, build options, and server configuration.

use anyhow::{Context, Result};
use cargo_metadata::MetadataCommand;
use serde::Deserialize;
use std::collections::HashMap;

pub mod tailwind;

/// The root configuration structure for a MontRS project.
///
/// Corresponds to the `mont.toml` file.
#[derive(Debug, Deserialize, Clone, Default)]
pub struct MontConfig {
    /// Project identity and core settings.
    #[serde(default)]
    pub project: ProjectConfig,
    /// Build-related configuration (target, assets, etc.).
    #[serde(default)]
    pub build: BuildConfig,
    /// Development server settings.
    #[serde(default)]
    pub serve: ServeConfig,
    /// E2E testing configuration.
    #[serde(default)]
    pub e2e: E2eConfig,
    /// Custom task definitions.
    #[serde(default)]
    pub tasks: HashMap<String, TaskConfig>,
}

/// Project metadata and feature flags.
#[derive(Debug, Deserialize, Clone)]
pub struct ProjectConfig {
    /// The name of the project (defaults to package name).
    #[serde(default = "default_app_name")]
    pub name: String,
    
    // Internal fields for cargo-leptos compatibility
    #[serde(skip)]
    pub verbose: u8,
    #[serde(skip)]
    pub log: Vec<String>,
    #[serde(skip)]
    pub release: bool,
    #[serde(skip)]
    pub hot_reload: bool,
    #[serde(skip)]
    pub precompress: bool,
    #[serde(skip)]
    pub wasm_debug: bool,
    #[serde(skip)]
    pub js_minify: bool,
    #[serde(skip)]
    pub split: bool,
    #[serde(skip)]
    pub frontend_only: bool,
    #[serde(skip)]
    pub server_only: bool,
    #[serde(skip)]
    pub features: Vec<String>,
    #[serde(skip)]
    pub tailwind_style: Option<TailwindStyle>,
}

/// Tailwind CSS integration style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TailwindStyle {
    /// Automatically detect style.
    #[default]
    Auto,
    /// Use configuration from `mont.toml`.
    Toml,
    /// Use Tailwind v4 conventions.
    V4,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            name: default_app_name(),
            verbose: 0,
            log: Vec::new(),
            release: false,
            hot_reload: false,
            precompress: false,
            wasm_debug: false,
            js_minify: true,
            split: false,
            frontend_only: false,
            server_only: false,
            features: Vec::new(),
            tailwind_style: None,
        }
    }
}

fn default_app_name() -> String {
    "app".to_string()
}

/// Build configuration settings.
#[derive(Debug, Deserialize, Clone)]
pub struct BuildConfig {
    /// The HTML file to use as the index page (default: "index.html").
    #[serde(default = "default_target")]
    pub target: String,
    /// The directory to output build artifacts (default: "dist").
    #[serde(default = "default_dist")]
    pub dist: String,
    /// The root directory for the site (default: "target/site").
    #[serde(default = "default_site_root")]
    pub site_root: String,
    /// The name of the WASM package directory (default: "pkg").
    #[serde(default = "default_site_pkg_name")]
    pub site_pkg_name: String,
    /// Optional directory containing static assets.
    #[serde(default = "default_assets_dir")]
    pub assets_dir: Option<String>,
    /// Path to the Tailwind CSS input file.
    #[serde(default)]
    pub tailwind_input_file: Option<String>,
    /// Path to the Tailwind CSS config file.
    #[serde(rename = "tailwind-config-file")]
    pub tailwind_config_file: Option<String>,
    /// Path to the main style file (e.g., CSS/SCSS).
    #[serde(rename = "style-file")]
    pub style_file: Option<String>,
    /// Browser compatibility query (default: "defaults").
    #[serde(default = "default_browserquery")]
    pub browserquery: String,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            target: default_target(),
            dist: default_dist(),
            site_root: default_site_root(),
            site_pkg_name: default_site_pkg_name(),
            assets_dir: None,
            tailwind_input_file: None,
            tailwind_config_file: None,
            style_file: None,
            browserquery: default_browserquery(),
        }
    }
}

fn default_browserquery() -> String {
    "defaults".to_string()
}

fn default_target() -> String {
    "index.html".to_string()
}
fn default_dist() -> String {
    "dist".to_string()
}
fn default_site_root() -> String {
    "target/site".to_string()
}
fn default_site_pkg_name() -> String {
    "pkg".to_string()
}
fn default_assets_dir() -> Option<String> {
    None
}

/// Development server configuration.
#[derive(Debug, Deserialize, Clone)]
pub struct ServeConfig {
    /// The port to listen on (default: 8080).
    #[serde(default = "default_port")]
    pub port: u16,
    /// The address to bind to (default: "127.0.0.1").
    #[serde(default = "default_addr")]
    pub addr: String,
}

impl Default for ServeConfig {
    fn default() -> Self {
        Self {
            port: default_port(),
            addr: default_addr(),
        }
    }
}

fn default_port() -> u16 {
    8080
}
fn default_addr() -> String {
    "127.0.0.1".to_string()
}

/// E2E testing configuration.
#[derive(Debug, Deserialize, Clone, Default)]
pub struct E2eConfig {
    /// Run browsers in headless mode.
    #[serde(default)]
    pub headless: Option<bool>,
    /// Browser to use (chromium, firefox, webkit).
    #[serde(default)]
    pub browser: Option<String>,
    /// Base URL for tests (overrides automatic detection).
    #[serde(default)]
    pub base_url: Option<String>,
}

/// Configuration for custom tasks.
#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum TaskConfig {
    /// A simple command string.
    Simple(String),
    /// A detailed task definition.
    Detailed {
        /// The command to execute.
        command: String,
        /// Description of the task.
        #[serde(default)]
        description: Option<String>,
        /// Category for grouping tasks.
        #[serde(default)]
        category: Option<String>,
        /// List of dependent tasks to run before this one.
        #[serde(default)]
        dependencies: Vec<String>,
        /// Environment variables to set for this task.
        #[serde(default)]
        env: HashMap<String, String>,
    },
}

impl MontConfig {
    /// Loads configuration from a specific file.
    pub fn from_file(path: impl AsRef<std::path::Path>) -> Result<Self> {
        let content = std::fs::read_to_string(path.as_ref())
            .with_context(|| format!("Failed to read config file: {}", path.as_ref().display()))?;
        let mut config: Self = toml::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {}", path.as_ref().display()))?;

        // Try to resolve project name if it's default
        if config.project.name == "app" {
            if let Ok(metadata) = MetadataCommand::new().exec() {
                if let Some(root) = metadata.root_package() {
                    config.project.name = root.name.clone();
                }
            }
        }

        Ok(config)
    }

    /// Loads configuration from `mont.toml` in the current directory.
    ///
    /// If the file is missing, returns default configuration.
    /// Also attempts to resolve the project name from `Cargo.toml`.
    pub fn load() -> Result<Self> {
        if std::path::Path::new("mont.toml").exists() {
            Self::from_file("mont.toml")
        } else {
            let mut config = Self::default();
            // Try to resolve project name
            if let Ok(metadata) = MetadataCommand::new().exec() {
                if let Some(root) = metadata.root_package() {
                    config.project.name = root.name.clone();
                }
            }
            Ok(config)
        }
    }

    // to_leptos_config removed as we now use cargo-leptos CLI wrapper
}
