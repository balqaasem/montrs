//! Configuration module for MontRS.
//!
//! This module defines the structure of the `mont.toml` configuration file
//! and handles loading/parsing logic. It serves as the central source of truth
//! for project settings, build options, and server configuration.

use anyhow::{Context, Result};
use camino::{Utf8Path, Utf8PathBuf};
use cargo_metadata::MetadataCommand;
use serde::Deserialize;
use std::{collections::HashMap, net::SocketAddr, sync::Arc};

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
    pub log: Vec<cargo_leptos::config::Log>,
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
    /// Loads configuration from `mont.toml` in the current directory.
    ///
    /// If the file is missing, returns default configuration.
    /// Also attempts to resolve the project name from `Cargo.toml`.
    pub fn load() -> Result<Self> {
        let metadata = MetadataCommand::new()
            .exec()
            .map_err(|e| anyhow::anyhow!("Failed to load cargo metadata: {}", e))?;

        let mut config = if let Ok(content) = std::fs::read_to_string("mont.toml") {
            toml::from_str(&content).context("Failed to parse mont.toml")?
        } else {
            Self::default()
        };

        if let Some(root) = metadata.root_package()
            && config.project.name == "app"
        {
            config.project.name = root.name.clone();
        }

        Ok(config)
    }

    /// Converts MontRS configuration to `cargo-leptos` configuration.
    ///
    /// This allows `cargo-mont` to reuse `cargo-leptos` functionality while
    /// maintaining its own configuration format.
    pub fn to_leptos_config(&self, watch: bool) -> Result<cargo_leptos::config::Config> {
        let metadata = MetadataCommand::new()
            .exec()
            .map_err(|e| anyhow::anyhow!("Failed to load cargo metadata: {}", e))?;

        let mut opts = cargo_leptos::config::Opts::default();
        opts.project = Some(self.project.name.clone());
        opts.verbose = self.project.verbose;
        opts.log = self.project.log.clone();
        opts.release = self.project.release;
        opts.hot_reload = self.project.hot_reload;
        opts.precompress = self.project.precompress;
        opts.wasm_debug = self.project.wasm_debug;
        opts.js_minify = self.project.js_minify;
        opts.split = self.project.split;
        opts.frontend_only = self.project.frontend_only;
        opts.server_only = self.project.server_only;
        opts.features = self.project.features.clone();

        let mut proj_conf = cargo_leptos::config::ProjectConfig {
            output_name: self.project.name.clone(),
            site_addr: format!("{}:{}", self.serve.addr, self.serve.port)
                .parse::<SocketAddr>()
                .context("Invalid site address")?,
            site_root: Utf8PathBuf::from(&self.build.site_root),
            site_pkg_dir: Utf8PathBuf::from(&self.build.site_pkg_name),
            style_file: self.build.style_file.as_ref().map(Utf8PathBuf::from),
            hash_file_name: None,
            hash_files: false,
            tailwind_input_file: self
                .build
                .tailwind_input_file
                .as_ref()
                .map(Utf8PathBuf::from),
            tailwind_config_file: self
                .build
                .tailwind_config_file
                .as_ref()
                .map(Utf8PathBuf::from),
            assets_dir: self.build.assets_dir.as_ref().map(Utf8PathBuf::from),
            js_dir: None,
            js_minify: true,
            watch_additional_files: None,
            reload_port: 3001,
            end2end_cmd: None,
            end2end_dir: None,
            browserquery: self.build.browserquery.clone(),
            bin_target: String::new(),
            bin_target_triple: None,
            bin_target_dir: None,
            bin_cargo_command: None,
            bin_cargo_args: None,
            bin_exe_name: None,
            features: Vec::new(),
            lib_features: Vec::new(),
            lib_default_features: true,
            lib_cargo_args: None,
            bin_features: Vec::new(),
            bin_default_features: true,
            server_fn_prefix: None,
            disable_server_fn_hash: false,
            disable_erase_components: false,
            always_erase_components: false,
            server_fn_mod_path: false,
            config_dir: Utf8PathBuf::from("."),
            tmp_dir: metadata.target_directory.join("tmp"),
            separate_front_target_dir: None,
            lib_profile_dev: None,
            lib_profile_release: None,
            bin_profile_dev: None,
            bin_profile_release: None,
            wasm_opt_features: None,
        };

        // If cargo-leptos has these types and functions as public, we can use them:
        // Note: ProjectDefinition is not usually public but we can try Constructing it or using a public path if available.
        // Actually, Project has a public constructor-like logic in Project::resolve.

        // Since we want to ENSURE our mont.toml data is used, we have to construct the Project fields.

        let project_def = cargo_leptos::config::ProjectDefinition {
            name: self.project.name.clone(),
            bin_package: self.project.name.clone(),
            lib_package: self.project.name.clone(),
        };

        let cwd = Utf8PathBuf::from(".");

        let lib =
            cargo_leptos::config::LibPackage::resolve(&opts, &metadata, &project_def, &proj_conf)
                .map_err(|e| anyhow::anyhow!("Failed to resolve lib package: {}", e))?;

        let bin = cargo_leptos::config::BinPackage::resolve(
            &opts,
            &metadata,
            &project_def,
            &proj_conf,
            None,
        )
        .map_err(|e| anyhow::anyhow!("Failed to resolve bin package: {}", e))?;

        let style = cargo_leptos::config::StyleConfig::new(&proj_conf)
            .map_err(|e| anyhow::anyhow!("Failed to resolve style config: {}", e))?;

        let site = Arc::new(cargo_leptos::service::site::Site::new(&proj_conf));

        let hash_file =
            cargo_leptos::config::HashFile::new(Some(&metadata.workspace_root), &bin, None);

        let project = cargo_leptos::config::Project {
            working_dir: metadata.workspace_root.clone(),
            name: self.project.name.clone(),
            lib,
            bin,
            style,
            watch,
            release: opts.release,
            precompress: opts.precompress,
            hot_reload: opts.hot_reload,
            wasm_debug: opts.wasm_debug,
            site,
            end2end: cargo_leptos::config::End2EndConfig::resolve(&proj_conf),
            assets: cargo_leptos::config::AssetsConfig::resolve(&proj_conf),
            js_dir: Utf8PathBuf::from("src"),
            watch_additional_files: Vec::new(),
            hash_file,
            hash_files: false,
            js_minify: true,
            split: false,
            server_fn_prefix: None,
            disable_server_fn_hash: false,
            disable_erase_components: false,
            always_erase_components: false,
            server_fn_mod_path: false,
            wasm_opt_features: None,
            build_frontend_only: false,
            build_server_only: false,
            clear_terminal_on_rebuild: false,
        };

        Ok(cargo_leptos::config::Config {
            working_dir: metadata.workspace_root,
            projects: vec![Arc::new(project)],
            cli: opts,
            watch,
        })
    }
}
