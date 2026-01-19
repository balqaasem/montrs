use anyhow::{Context, Result};
use cargo_metadata::MetadataCommand;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct MontConfig {
    #[serde(default)]
    pub project: ProjectConfig,
    #[serde(default)]
    pub build: BuildConfig,
    #[serde(default)]
    pub serve: ServeConfig,
    #[serde(default)]
    pub tasks: HashMap<String, TaskConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ProjectConfig {
    #[serde(default = "default_app_name")]
    pub name: String,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            name: default_app_name(),
        }
    }
}

fn default_app_name() -> String {
    "app".to_string()
}

#[derive(Debug, Deserialize, Clone)]
pub struct BuildConfig {
    #[serde(default = "default_target")]
    pub target: String,
    #[serde(default = "default_dist")]
    pub dist: String,
    #[serde(default = "default_site_root")]
    pub site_root: String,
    #[serde(default = "default_site_pkg_name")]
    pub site_pkg_name: String,
    #[serde(default = "default_assets_dir")]
    pub assets_dir: Option<String>,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            target: default_target(),
            dist: default_dist(),
            site_root: default_site_root(),
            site_pkg_name: default_site_pkg_name(),
            assets_dir: None,
        }
    }
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

#[derive(Debug, Deserialize, Clone)]
pub struct ServeConfig {
    #[serde(default = "default_port")]
    pub port: u16,
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

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum TaskConfig {
    Simple(String),
    Detailed {
        command: String,
        #[serde(default)]
        description: Option<String>,
        #[serde(default)]
        category: Option<String>,
        #[serde(default)]
        dependencies: Vec<String>,
        #[serde(default)]
        env: HashMap<String, String>,
    },
}

impl MontConfig {
    pub fn load() -> Result<Self> {
        let metadata = MetadataCommand::new()
            .exec()
            .map_err(|e| anyhow::anyhow!("Failed to load cargo metadata: {}", e))?;

        let mut config = if let Ok(content) = std::fs::read_to_string("mont.toml") {
            toml::from_str(&content).context("Failed to parse mont.toml")?
        } else {
            Self::default()
        };

        if let Some(root) = metadata.root_package() {
            if config.project.name == "app" {
                config.project.name = root.name.clone();
            }
        }

        Ok(config)
    }
}
